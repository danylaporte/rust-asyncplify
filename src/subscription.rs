use consumer::*;
use std::marker::PhantomData;
use stream::*;

/// Represents a subscription to a `Stream`
pub struct Subscription<I> {
    closed: bool,
    marker: PhantomData<I>,
}

impl<I> Subscription<I> {
    /// Closes the `Subscription<I>`
    pub fn close(mut self) {
        self.closed = true;
    }
}

impl<T> Consumer<T> for Subscription<T> {
    fn emit(&mut self, _: T) -> bool {
        !self.closed
    }
}

pub trait SubscribableStream<T>: Stream<T> {
    fn subscribe(self)
        where Self: Sized
    {
        self.consume(Subscription {
            closed: false,
            marker: PhantomData::<T>,
        });
    }
}

impl<S, T> SubscribableStream<T> for S where S: Stream<T> {}