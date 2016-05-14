use consumer::*;
use producer::*;
use std::cmp::PartialOrd;
use std::marker::PhantomData;
use std::mem::replace;
use std::rc::Rc;
use stream::*;

struct MaxByKeyState<C, T, F, K>
    where C: Consumer<T>,
          F: FnMut(&T) -> K,
          K: PartialOrd
{
    consumer: C,
    f: F,
    value: Option<(K, T)>,
}

impl<C, T, F, K> Consumer<T> for MaxByKeyState<C, T, F, K>
    where C: Consumer<T>,
          F: FnMut(&T) -> K,
          K: PartialOrd
{
    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: T) {
        let k = (self.f)(&item);
        if let Some(ref value) = self.value {
            if value.0 > k {
                return;
            }
        }

        self.value = Some((k, item));
    }
}

impl<C, T, F, K> Drop for MaxByKeyState<C, T, F, K>
    where C: Consumer<T>,
          F: FnMut(&T) -> K,
          K: PartialOrd
{
    fn drop(&mut self) {
        if let Some(value) = replace(&mut self.value, None) {
            self.consumer.emit(value.1);
        }
    }
}

pub struct MaxByKey<S, F, K> {
    f: F,
    marker_k: PhantomData<K>,
    stream: S,
}

impl<S, T, F, K> Stream<T> for MaxByKey<S, F, K>
    where S: Stream<T>,
          F: FnMut(&T) -> K,
          K: PartialOrd
{
    fn consume<C: Consumer<T>>(self, consumer: C) {
        self.stream.consume(MaxByKeyState {
            consumer: consumer,
            f: self.f,
            value: None,
        });
    }
}

impl<S, F, K> MaxByKey<S, F, K> {
    pub fn new(stream: S, f: F) -> Self {
        MaxByKey {
            f: f,
            marker_k: PhantomData::<K>,
            stream: stream,
        }
    }
}