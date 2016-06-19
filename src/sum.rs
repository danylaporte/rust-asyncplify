use consumer::*;
use std::mem::replace;
use std::ops::AddAssign;
use stream::*;

struct SumState<C, T>
    where C: Consumer<T>,
          T: AddAssign + Default
{
    consumer: C,
    value: T,
}

impl<C, T> Consumer<T> for SumState<C, T>
    where C: Consumer<T>,
          T: AddAssign + Default
{
    fn emit(&mut self, item: T) -> bool {
        self.value += item;
        true
    }
}

impl<C, T> Drop for SumState<C, T>
    where C: Consumer<T>,
          T: AddAssign + Default
{
    fn drop(&mut self) {
        let value = replace(&mut self.value, Default::default());
        self.consumer.emit(value);
    }
}

/// Sum all the items and emit only a single item as the total.
///
/// This struct is created by the [`sum()`](./trait.Stream.html#method.sum) method
/// on [Stream](./trait.Stream.html). See its documentation for more.
#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct Sum<S> {
    stream: S,
}

impl<S> Stream for Sum<S>
    where S: Stream,
          S::Item: AddAssign + Default
{
    type Item = S::Item;

    fn consume<C>(self, consumer: C)
        where C: Consumer<Self::Item>
    {
        self.stream.consume(SumState {
            consumer: consumer,
            value: Default::default(),
        });
    }
}

impl<S> Sum<S> {
    pub fn new(stream: S) -> Self {
        Sum { stream: stream }
    }
}
