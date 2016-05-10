use consumer::*;
use producer::*;
use std::rc::Rc;
use stream::*;

struct SkipState<C> {
    consumer: C,
    count: u64,
}

pub struct Skip<S> {
    count: u64,
    stream: S,
}

impl<C, T> Consumer<T> for SkipState<C>
    where C: Consumer<T>
{
    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: T) {
        if self.count > 0 {
            self.count -= 1;
        } else {
            self.consumer.emit(item);
        }
    }
}

impl<S, T> Stream<T> for Skip<S>
    where S: Stream<T>
{
    fn consume<C: Consumer<T>>(self, consumer: C) {
        self.stream.consume(SkipState {
            consumer: consumer,
            count: self.count,
        });
    }
}

pub trait SkipStream<I>: Stream<I> {
    /// Ignore the first X values from the stream
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    ///
    /// let vec = (0..10)
    ///     .to_stream()
    ///     .skip(3)
    ///     .into_vec();
    /// assert!(vec == [3, 4, 5, 6, 7, 8, 9], "vec = {:?}", vec);
    /// ```
    fn skip(self, count: u64) -> Skip<Self>
        where Self: Sized
    {
        Skip {
            count: count,
            stream: self,
        }
    }
}

impl<S, T> SkipStream<T> for S where S: Stream<T> {}