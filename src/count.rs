use consumer::*;
use producer::*;
use std::marker::PhantomData;
use std::rc::Rc;
use stream::*;

struct CountState<C>
    where C: Consumer<u64>
{
    consumer: C,
    value: u64,
}

impl<C, T> Consumer<T> for CountState<C>
    where C: Consumer<u64>
{
    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, _: T) {
        self.value += 1;
    }
}

impl<C> Drop for CountState<C>
    where C: Consumer<u64>
{
    fn drop(&mut self) {
        self.consumer.emit(self.value);
    }
}

pub struct Count<S, T> {
    stream: S,
    marker_t: PhantomData<T>,
}

impl<S, T> Stream<u64> for Count<S, T>
    where S: Stream<T>
{
    fn consume<C: Consumer<u64>>(self, consumer: C) {
        self.stream.consume(CountState {
            consumer: consumer,
            value: 0,
        });
    }
}

pub trait CountStream<T>: Stream<T> {
    /// Count the number ofthe item received.
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    ///
    /// let vec = (0..10)
    ///     .to_stream()
    ///     .count()
    ///     .into_vec();
    /// assert!(vec == [10], "vec = {:?}", vec);
    /// ```
    fn count(self) -> Count<Self, T>
        where Self: Sized
    {
        Count {
            stream: self,
            marker_t: PhantomData::<T>,
        }
    }
}

impl<S, T> CountStream<T> for S where S: Stream<T> {}