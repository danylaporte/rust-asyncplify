use consumer::*;
use std::cmp::Eq;
use std::marker::PhantomData;
use stream::*;

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct DedupByKey<S, F, K> {
    key_selector: F,
    marker_k: PhantomData<K>,
    stream: S,
}

impl<S, F, K, T> Stream<T> for DedupByKey<S, F, K>
    where S: Stream<T>,
          F: FnMut(&T) -> K,
          K: Eq
{
    fn consume<C: Consumer<T>>(self, consumer: C) {
        self.stream.consume(DedupByKeyState {
            consumer: consumer,
            key_selector: self.key_selector,
            last: None,
        })
    }
}

struct DedupByKeyState<C, F, K> {
    consumer: C,
    key_selector: F,
    last: Option<K>,
}

impl<C, F, K, T> Consumer<T> for DedupByKeyState<C, F, K>
    where C: Consumer<T>,
          K: Eq,
          F: FnMut(&T) -> K
{
    fn emit(&mut self, item: T) -> bool {
        let key = (self.key_selector)(&item);

        if let Some(ref old_key) = self.last {
            if key == *old_key {
                return true;
            }
        }

        self.last = Some(key);
        self.consumer.emit(item)
    }
}

impl<S, F, K> DedupByKey<S, F, K> {
    pub fn new(stream: S, key_selector: F) -> Self {
        DedupByKey {
            key_selector: key_selector,
            marker_k: PhantomData::<K>,
            stream: stream,
        }
    }
}
