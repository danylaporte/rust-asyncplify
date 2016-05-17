use consumer::*;
use std::cmp::Eq;
use stream::*;

pub struct Dedup<S> {
    stream: S,
}

impl<S, T> Stream<T> for Dedup<S>
    where S: Stream<T>,
          T: Clone + Eq
{
    fn consume<C: Consumer<T>>(self, consumer: C) {
        self.stream.consume(DedupState {
            consumer: consumer,
            last: None,
        })
    }
}

struct DedupState<C, T> {
    consumer: C,
    last: Option<T>,
}

impl<C, T> Consumer<T> for DedupState<C, T>
    where C: Consumer<T>,
          T: Clone + Eq
{
    fn emit(&mut self, item: T) -> bool {
        if let Some(ref last) = self.last {
            if *last == item {
                return true;
            }
        }
        
        self.last = Some(item.clone());
        self.consumer.emit(item)
    }
}

impl<S> Dedup<S> {
    pub fn new(stream: S) -> Self {
        Dedup { stream: stream }
    }
}