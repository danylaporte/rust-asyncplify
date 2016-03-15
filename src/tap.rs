use std::marker::PhantomData;
use std::rc::Rc;
use consumer::*;
use producer::*;
use stream::*;

struct TapState<C, F, T> {
    consumer: C,
    func: F,
    marker: PhantomData<T>,
}

pub struct TapStream<S, F> {
    stream: S,
    func: F,
}

impl<C, F, T> Consumer for TapState<C, F, T>
    where C: Consumer<Item = T>,
          F: FnMut(&T)
{
    type Item = T;

    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: Self::Item) {
        (self.func)(&item);
        self.consumer.emit(item);
    }

    fn end(self) {
        self.consumer.end();
    }
}

impl<S, F> Stream for TapStream<S, F>
    where S: Stream,
          F: FnMut(&S::Item)
{
    type Item = S::Item;

    fn consume<C>(self, consumer: C)
        where C: Consumer<Item = Self::Item>
    {
        self.stream.consume(TapState {
            consumer: consumer,
            func: self.func,
            marker: PhantomData::<Self::Item>,
        });
    }
}

pub trait  TappableStream : Stream {
    fn tap<F>(self, func: F) -> TapStream<Self, F>
        where F: FnMut(&Self::Item),
              Self: Sized
    {
        TapStream {
            stream: self,
            func: func,
        }
    }
}

impl<S> TappableStream for S where S: Stream
{}
