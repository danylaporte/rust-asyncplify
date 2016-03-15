use std::marker::PhantomData;
use std::rc::Rc;
use consumer::*;
use producer::*;
use stream::*;

pub struct Subscription<I> {
    producer: Option<Rc<Producer>>,
    marker: PhantomData<I>,
}

impl<I> Consumer for Subscription<I> {
    type Item = I;

    fn init(&mut self, producer: Rc<Producer>) {
        self.producer = Some(producer);
    }

    fn emit(&mut self, _: Self::Item) {}

    fn end(self) {}
}

pub trait  SubscribableStream : Stream {
    fn subscribe(self)
        where Self: Sized
    {
        self.consume(Subscription {
            producer: None,
            marker: PhantomData::<Self::Item>,
        });
    }
}

impl<S> SubscribableStream for S where S: Stream
{}
