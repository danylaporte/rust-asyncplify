use consumer::*;
use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;
use stream::*;

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct Unique<S> {
    stream: S,
}

impl<S> Stream for Unique<S>
    where S: Stream,
          S::Item: Clone + Eq + Hash
{
    type Item = S::Item;

    fn consume<C>(self, consumer: C)
        where C: Consumer<Self::Item>
    {
        self.stream.consume(UniqueState {
            consumer: consumer,
            hashset: HashSet::new(),
        });
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
