use consumer::*;
use producer::*;
use std::rc::Rc;
use stream::*;

struct SkipState<C> {
    consumer: C,
    count: u64,
}

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct Skip<S> {
    count: u64,
    stream: S,
}

impl<C, T> Consumer<T> for SkipState<C>
    where C: Consumer<T>
{
    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: T) {
        if self.count > 0 {
            self.count -= 1;
        } else {
            self.consumer.emit(item);
        }
    }
}

impl<S, T> Stream<T> for Skip<S>
    where S: Stream<T>
{
    fn consume<C: Consumer<T>>(self, consumer: C) {
        self.stream.consume(SkipState {
            consumer: consumer,
            count: self.count,
        });
    }
}

impl<S> Skip<S> {
    pub fn new(stream: S, count: u64) -> Self {
        Skip {
            count: count,
            stream: stream,
        }
    }
}