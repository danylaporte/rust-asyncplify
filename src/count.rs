use consumer::*;
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
pub struct Count<S> {
    stream: S,
}

impl<S> Stream for Count<S>
    where S: Stream
{
    type Item = u64;

    fn consume<C>(self, consumer: C)
        where C: Consumer<u64>
    {
        self.stream.consume(CountState {
            consumer: consumer,
            value: 0,
        });
    }
}

impl<S> Count<S> {
    pub fn new(stream: S) -> Self {
        Count { stream: stream }
    }
}
