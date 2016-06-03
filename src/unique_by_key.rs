use consumer::*;
use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;
use std::marker::PhantomData;
use stream::*;

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct UniqueByKey<S, F, K> {
    key_selector: F,
    marker_k: PhantomData<K>,
    stream: S,
}

impl<S, F, K, T> Stream<T> for UniqueByKey<S, F, K>
    where S: Stream<T>,
          F: FnMut(&T) -> K,
          K: Eq + Hash
{
    fn consume<C: Consumer<T>>(self, consumer: C) {
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

impl<S, F, K> UniqueByKey<S, F, K> {
    pub fn new(stream: S, key_selector: F) -> Self {
        UniqueByKey {
            key_selector: key_selector,
            marker_k: PhantomData::<K>,
            stream: stream,
        }
    }
}
