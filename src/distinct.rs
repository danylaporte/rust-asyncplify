use consumer::*;
use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;
use stream::*;

pub struct Distinct<S> {
    stream: S,
}

impl<S, T> Stream<T> for Distinct<S>
    where S: Stream<T>,
          T: Clone + Eq + Hash
{
    fn consume<C: Consumer<T>>(self, consumer: C) {
        self.stream.consume(DistinctState {
            consumer: consumer,
            hashset: HashSet::new(),
        })
    }
}

struct DistinctState<C, T> {
    consumer: C,
    hashset: HashSet<T>,
}

impl<C, T> Consumer<T> for DistinctState<C, T>
    where C: Consumer<T>,
          T: Clone + Eq + Hash
{
    fn emit(&mut self, item: T) -> bool {
        !self.hashset.insert(item.clone()) || self.consumer.emit(item)
    }
}

impl<S> Distinct<S> {
    pub fn new(stream: S) -> Self {
        Distinct { stream: stream }
    }
}