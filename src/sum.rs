use consumer::*;
use producer::*;
use std::mem::replace;
use std::ops::AddAssign;
use std::rc::Rc;
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
    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: T) {
        self.value += item;
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

pub struct Sum<S> {
    stream: S,
}

impl<S, T> Stream<T> for Sum<S>
    where S: Stream<T>,
          T: AddAssign + Default
{
    fn consume<C: Consumer<T>>(self, consumer: C) {
        self.stream.consume(SumState {
            consumer: consumer,
            value: Default::default(),
        });
    }
}

pub trait SumStream<T>: Stream<T> {
    /// Calculate the sum of the item received.
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    /// let vec = (0..10)
    ///     .to_stream()
    ///     .sum()
    ///     .into_vec();
    /// assert!(vec == [45], "vec = {:?}", vec);
    /// ```
    fn sum(self) -> Sum<Self>
        where Self: Sized
    {
        Sum { stream: self }
    }
}

impl<S, T> SumStream<T> for S where S: Stream<T> {}