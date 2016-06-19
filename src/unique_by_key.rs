use consumer::*;
use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;
use stream::*;

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct UniqueByKey<S, F> {
    key_selector: F,
    stream: S,
}

impl<S, F, K> Stream for UniqueByKey<S, F>
    where S: Stream,
          F: FnMut(&S::Item) -> K,
          K: Eq + Hash
{
    type Item = S::Item;

    fn consume<C>(self, consumer: C)
        where C: Consumer<Self::Item>
    {
        self.stream.consume(UniqueByKeyState {
            consumer: consumer,
            hashset: HashSet::new(),
            key_selector: self.key_selector,
        })
    }
}

struct UniqueByKeyState<C, F, K> {
    consumer: C,
    hashset: HashSet<K>,
    key_selector: F,
}

impl<C, F, K, T> Consumer<T> for UniqueByKeyState<C, F, K>
    where C: Consumer<T>,
          F: FnMut(&T) -> K,
          K: Eq + Hash
{
    fn emit(&mut self, item: T) -> bool {
        let key = (self.key_selector)(&item);
        !self.hashset.insert(key) || self.consumer.emit(item)
    }
}

impl<S, F> UniqueByKey<S, F> {
    pub fn new(stream: S, key_selector: F) -> Self {
        UniqueByKey {
            key_selector: key_selector,
            stream: stream,
        }
    }
}
