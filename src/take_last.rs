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

impl<S, T> Stream<T> for TakeLast<S>
    where S: Stream<T>
{
    fn consume<C: Consumer<T>>(self, consumer: C) {
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
