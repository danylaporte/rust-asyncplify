use consumer::*;
use producer::*;
use std::marker::PhantomData;
use std::mem;
use std::option::*;
use std::rc::Rc;
use stream::*;

/// Represents a subscription to a `Stream`
pub struct Subscription<I> {
    producer: Option<Rc<Producer>>,
    marker: PhantomData<I>,
}

impl<I> Subscription<I> {
    /// Closes the `Subscription<I>`
    pub fn close(mut self) {
        if let Some(p) = mem::replace(&mut self.producer, None) {
            p.close();
        }
    }
}

impl<T> Consumer<T> for Subscription<T> {
    fn init(&mut self, producer: Rc<Producer>) {
        self.producer = Some(producer);
    }
    fn emit(&mut self, _: T) {}
}

pub trait SubscribableStream<T>: Stream<T> {
    fn subscribe(self)
        where Self: Sized
    {
        self.consume(Subscription {
            producer: None,
            marker: PhantomData::<T>,
        });
    }
}

impl<S, T> SubscribableStream<T> for S where S: Stream<T> {}