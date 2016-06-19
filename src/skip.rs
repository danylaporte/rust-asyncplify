use consumer::*;
use stream::*;

struct SkipState<C> {
    consumer: C,
    count: u64,
}

/// Ignores the first n items.
///
/// This struct is created by the [skip()](./trait.Stream.html#method.skip)
/// method on [Stream](./trait.Stream.html).
/// See its documentation for more.
#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct Skip<S> {
    count: u64,
    stream: S,
}

impl<C, T> Consumer<T> for SkipState<C>
    where C: Consumer<T>
{
    fn emit(&mut self, item: T) -> bool {
        if self.count > 0 {
            self.count -= 1;
            true
        } else {
            self.consumer.emit(item)
        }
    }
}

impl<S> Stream for Skip<S>
    where S: Stream
{
    type Item = S::Item;

    fn consume<C>(self, consumer: C)
        where C: Consumer<Self::Item>
    {
        self.stream.consume(SkipState {
            consumer: consumer,
            count: self.count,
        });
    }
}

impl<S> Skip<S> {
    pub fn new(stream: S, count: u64) -> Self {
        Skip {
            count: count,
            stream: stream,
        }
    }
}
