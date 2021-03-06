use consumer::*;
use std::collections::VecDeque;
use stream::*;

struct TakeLastState<C, T>
    where C: Consumer<T>
{
    consumer: C,
    count: usize,
    queue: VecDeque<T>,
}

/// Emit the last n items of a stream.
///
/// This struct is created by the
/// [`take_last()`](./trait.Stream.html#method.take_last) method on
/// [Stream](./trait.Stream.html). See its documentation for more.
#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct TakeLast<S> {
    count: usize,
    stream: S,
}

impl<C, T> Consumer<T> for TakeLastState<C, T>
    where C: Consumer<T>
{
    fn emit(&mut self, item: T) -> bool {
        if self.count == self.queue.len() {
            self.queue.pop_front();
        }
        self.queue.push_back(item);
        true
    }
}

impl<C, T> Drop for TakeLastState<C, T>
    where C: Consumer<T>
{
    fn drop(&mut self) {
        for i in self.queue.drain(..) {
            if !self.consumer.emit(i) {
                break;
            }
        }
    }
}

impl<S> Stream for TakeLast<S>
    where S: Stream
{
    type Item = S::Item;

    fn consume<C>(self, consumer: C)
        where C: Consumer<Self::Item>
    {
        if self.count == 0 {
            self.stream.consume(consumer);
        } else {
            self.stream.consume(TakeLastState {
                consumer: consumer,
                count: self.count,
                queue: VecDeque::new(),
            });
        }
    }
}

impl<S> TakeLast<S> {
    pub fn new(stream: S, count: usize) -> Self {
        TakeLast {
            count: count,
            stream: stream,
        }
    }
}
