use consumer::*;
use stream::*;

struct TakeState<C> {
    consumer: C,
    count: u64,
}

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct Take<S> {
    count: u64,
    stream: S,
}

impl<C, T> Consumer<T> for TakeState<C>
    where C: Consumer<T>
{
    fn emit(&mut self, item: T) -> bool {
        if self.count > 0 {
            self.count -= 1;
            self.consumer.emit(item) && self.count > 0
        } else {
            false
        }
    }
}

impl<S, T> Stream<T> for Take<S>
    where S: Stream<T>
{
    fn consume<C: Consumer<T>>(self, consumer: C) {
        self.stream.consume(TakeState {
            consumer: consumer,
            count: self.count,
        });
    }
}

impl<S> Take<S> {
    pub fn new(stream: S, count: u64) -> Self {
        Take {
            count: count,
            stream: stream,
        }
    }
}