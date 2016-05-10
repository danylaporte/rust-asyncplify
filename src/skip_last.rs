use consumer::*;
use producer::*;
use std::collections::VecDeque;
use std::rc::Rc;
use stream::*;

struct SkipLastState<C, T>
    where C: Consumer<T>
{
    consumer: C,
    count: usize,
    queue: VecDeque<T>,
}

pub struct SkipLast<S> {
    count: usize,
    stream: S,
}

impl<C, T> Consumer<T> for SkipLastState<C, T>
    where C: Consumer<T>
{
    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: T) {
        if self.count == self.queue.len() {
            if let Some(old_item) = self.queue.pop_back() {
                self.consumer.emit(old_item);
            }
        }
        self.queue.push_front(item);
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

pub trait SkipLastStream<I>: Stream<I> {
    /// Ignores the last X values of the stream
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    ///
    /// let vec = (0..10)
    ///     .to_stream()
    ///     .skip_last(3)
    ///     .into_vec();
    /// assert!(vec == [0, 1, 2, 3, 4, 5, 6], "vec = {:?}", vec);
    /// ```
    fn skip_last(self, count: usize) -> SkipLast<Self>
        where Self: Sized
    {
        SkipLast {
            count: count,
            stream: self,
        }
    }
}

impl<S, T> SkipLastStream<T> for S where S: Stream<T> {}
