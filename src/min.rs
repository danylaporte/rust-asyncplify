use consumer::*;
use producer::*;
use std::cmp::PartialOrd;
use std::mem::replace;
use std::rc::Rc;
use stream::*;

struct MinState<C, T>
    where C: Consumer<T>,
          T: PartialOrd
{
    consumer: C,
    value: Option<T>,
}

impl<C, T> Consumer<T> for MinState<C, T>
    where C: Consumer<T>,
          T: PartialOrd
{
    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: T) {
        if let Some(ref value) = self.value {
            if value < &item {
                return;
            }
        }

        self.value = Some(item);
    }
}

impl<C, T> Drop for MinState<C, T>
    where C: Consumer<T>,
          T: PartialOrd
{
    fn drop(&mut self) {
        if let Some(value) = replace(&mut self.value, None) {
            self.consumer.emit(value);
        }
    }
}

pub struct Min<S> {
    stream: S,
}

impl<S, T> Stream<T> for Min<S>
    where S: Stream<T>,
          T: PartialOrd
{
    fn consume<C: Consumer<T>>(self, consumer: C) {
        self.stream.consume(MinState {
            consumer: consumer,
            value: None,
        });
    }
}

pub trait MinStream<T>: Stream<T> {
    /// Emit the item corresponding to the minimum value.
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    /// let mut value = 100;
    ///
    /// (0..10)
    ///     .to_stream()
    ///     .min()
    ///     .tap(|v| value = *v)
    ///     .subscribe();
    /// assert!(value == 0, "value = {:?}", value);
    /// ```

    fn min(self) -> Min<Self>
        where Self: Sized
    {
        Min { stream: self }
    }
}

impl<S, T> MinStream<T> for S where S: Stream<T> {}