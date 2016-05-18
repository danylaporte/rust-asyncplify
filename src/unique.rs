use consumer::*;
use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;
use stream::*;

pub struct Unique<S> {
    stream: S,
}

impl<S, T> Stream<T> for Unique<S>
    where S: Stream<T>,
          T: Clone + Eq + Hash
{
    fn consume<C: Consumer<T>>(self, consumer: C) {
        self.stream.consume(UniqueState {
            consumer: consumer,
            hashset: HashSet::new(),
        })
    }
}

struct UniqueState<C, T> {
    consumer: C,
    hashset: HashSet<T>,
}

impl<C, T> Consumer<T> for UniqueState<C, T>
    where C: Consumer<T>,
          T: Clone + Eq + Hash
{
    fn emit(&mut self, item: T) -> bool {
        !self.hashset.insert(item.clone()) || self.consumer.emit(item)
    }
}

impl<S> Unique<S> {
    pub fn new(stream: S) -> Self {
        Unique { stream: stream }
    }
}