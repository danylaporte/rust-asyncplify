use consumer::*;
use std::cmp::PartialOrd;
use std::marker::PhantomData;
use stream::*;

struct MinByKeyState<C, T, F, K>
    where C: Consumer<T>,
          F: FnMut(&T) -> K,
          K: PartialOrd
{
    consumer: C,
    f: F,
    value: Option<(K, T)>,
}

impl<C, T, F, K> Consumer<T> for MinByKeyState<C, T, F, K>
    where C: Consumer<T>,
          F: FnMut(&T) -> K,
          K: PartialOrd
{
    fn emit(&mut self, item: T) -> bool {

        let mut value = ((self.f)(&item), item);

        if let Some(current) = self.value.take() {
            if current.0 < value.0 {
                value = current;
            }
        }

        self.value = Some(value);
        true
    }
}

impl<C, T, F, K> Drop for MinByKeyState<C, T, F, K>
    where C: Consumer<T>,
          F: FnMut(&T) -> K,
          K: PartialOrd
{
    fn drop(&mut self) {
        if let Some(value) = self.value.take() {
            self.consumer.emit(value.1);
        }
    }
}

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct MinByKey<S, F, K> {
    f: F,
    marker_k: PhantomData<K>,
    stream: S,
}

impl<S, T, F, K> Stream<T> for MinByKey<S, F, K>
    where S: Stream<T>,
          F: FnMut(&T) -> K,
          K: PartialOrd
{
    fn consume<C: Consumer<T>>(self, consumer: C) {
        self.stream.consume(MinByKeyState {
            consumer: consumer,
            f: self.f,
            value: None,
        });
    }
}

impl<S, F, K> MinByKey<S, F, K> {
    pub fn new(stream: S, f: F) -> Self {
        MinByKey {
            f: f,
            marker_k: PhantomData::<K>,
            stream: stream,
        }
    }
}