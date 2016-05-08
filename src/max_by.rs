use consumer::*;
use producer::*;
use std::cmp::PartialOrd;
use std::marker::PhantomData;
use std::mem::replace;
use std::rc::Rc;
use stream::*;

struct MaxByState<C, T, F, K>
    where C: Consumer<T>,
          F: FnMut(&T) -> K,
          K: PartialOrd
{
    consumer: C,
    f: F,
    value: Option<(K, T)>,
}

impl<C, T, F, K> Consumer<T> for MaxByState<C, T, F, K>
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

impl<C, T, F, K> Drop for MaxByState<C, T, F, K>
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

pub struct MaxBy<S, F, K> {
    stream: S,
    f: F,
    marker_k: PhantomData<K>,
}

impl<S, T, F, K> Stream<T> for MaxBy<S, F, K>
    where S: Stream<T>,
          F: FnMut(&T) -> K,
          K: PartialOrd
{
    fn consume<C: Consumer<T>>(self, consumer: C) {
        self.stream.consume(MaxByState {
            consumer: consumer,
            f: self.f,
            value: None,
        });
    }
}

pub trait MaxByStream<T>: Stream<T> {
    /// Emit the item corresponding to the maximum value.
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    /// let mut value = 100;
    ///
    /// (0..10)
    ///     .to_stream()
    ///     .max_by(|v| 10 - *v)
    ///     .tap(|v| value = *v)
    ///     .subscribe();
    /// assert!(value == 0, "value = {:?}", value);
    /// ```

    fn max_by<F: FnMut(&T) -> K, K>(self, f: F) -> MaxBy<Self, F, K>
        where Self: Sized
    {
        MaxBy {
            stream: self,
            f: f,
            marker_k: PhantomData::<K>,
        }
    }
}

impl<S, T> MaxByStream<T> for S where S: Stream<T> {}