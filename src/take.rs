use consumer::*;
use producer::*;
use std::mem::replace;
use std::rc::Rc;
use stream::*;

struct TakeState<C> {
    consumer: C,
    count: u64,
    producer: Option<Rc<Producer>>,
}

pub struct Take<S> {
    count: u64,
    stream: S,
}

impl<C, T> Consumer<T> for TakeState<C>
    where C: Consumer<T>
{
    fn init(&mut self, producer: Rc<Producer>) {
        self.producer = Some(producer.clone());
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: T) {
        if self.count > 0 {
            self.count -= 1;
            self.consumer.emit(item);

            if self.count == 0 {
                if let Some(p) = replace(&mut self.producer, None) {
                    p.close();
                }
            }
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
            producer: None,
        });
    }
}

pub trait TakeStream<I>: Stream<I> {
    /// Take only the first X values of the stream and close the stream after
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    ///
    /// let vec = (0..10)
    ///     .to_stream()
    ///     .take(3)
    ///     .into_vec();
    /// assert!(vec == [0, 1, 2], "vec = {:?}", vec);
    /// ```
    fn take(self, count: u64) -> Take<Self>
        where Self: Sized
    {
        Take {
            count: count,
            stream: self,
        }
    }
}

impl<S, T> TakeStream<T> for S where S: Stream<T> {}