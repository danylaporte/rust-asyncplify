use consumer::*;
use std::collections::VecDeque;
use stream::*;

struct SkipLastState<C, T>
    where C: Consumer<T>
{
    consumer: C,
    count: usize,
    queue: VecDeque<T>,
}

/// Ignores the last n items.
///
/// This struct is created by the
/// [`skip_last()`](./trait.Stream.html#method.skip_last) method on
/// [Stream](./trait.Stream.html). See its documentation for more.
#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct SkipLast<S> {
    count: usize,
    stream: S,
}

impl<C, T> Consumer<T> for SkipLastState<C, T>
    where C: Consumer<T>
{
    fn emit(&mut self, item: T) -> bool {
        if self.count == self.queue.len() {
            if let Some(old_item) = self.queue.pop_back() {
                if !self.consumer.emit(old_item) {
                    return false;
                }
            }
        }
        self.queue.push_front(item);
        true
    }
}

impl<S, T> Stream<T> for SkipLast<S>
    where S: Stream<T>
{
    fn consume<C: Consumer<T>>(self, consumer: C) {
        if self.count == 0 {
            self.stream.consume(consumer);
        } else {
            self.stream.consume(SkipLastState {
                consumer: consumer,
                count: self.count,
                queue: VecDeque::new(),
            });
        }
    }
}

impl<S> SkipLast<S> {
    pub fn new(stream: S, count: usize) -> Self {
        SkipLast {
            count: count,
            stream: stream,
        }
    }
}
