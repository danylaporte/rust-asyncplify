use consumer::*;
use std::marker::PhantomData;
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
    fn emit(&mut self, _: T) -> bool {
        self.value += 1;
        true
    }
}

impl<C> Drop for CountState<C>
    where C: Consumer<u64>
{
    fn drop(&mut self) {
        self.consumer.emit(self.value);
    }
}

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
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

impl<S, T> Count<S, T> {
    pub fn new(stream: S) -> Self {
        Count {
            stream: stream,
            marker_t: PhantomData::<T>,
        }
    }
}