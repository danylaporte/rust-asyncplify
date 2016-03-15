use std::marker::PhantomData;
use std::rc::Rc;
use consumer::*;
use producer::*;
use stream::*;

struct MapState<C, F, I, O> {
    consumer: C,
    func: F,
    marker_i: PhantomData<I>,
    marker_o: PhantomData<O>,
}

pub struct MapStream<S, F, O> {
    stream: S,
    func: F,
    marker: PhantomData<O>,
}

impl<C, F, I, O> Consumer for MapState<C, F, I, O>
    where C: Consumer<Item = O>,
          F: Fn(I) -> C::Item
{
    type Item = I;

    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: Self::Item) {
        self.consumer.emit((self.func)(item));
    }

    fn end(self) {
        self.consumer.end();
    }
}

impl<S, F, O> Stream for MapStream<S, F, O>
    where S: Stream,
          F: Fn(S::Item) -> O
{
    type Item = O;

    fn consume<C>(self, consumer: C)
        where C: Consumer<Item = Self::Item>
    {
        self.stream.consume(MapState {
            consumer: consumer,
            func: self.func,
            marker_i: PhantomData::<S::Item>,
            marker_o: PhantomData::<Self::Item>,
        });
    }
}

pub trait  MappableStream : Stream {
    fn map<O, F>(self, func: F) -> MapStream<Self, F, O>
        where Self: Sized,
              F: Fn(Self::Item) -> O
    {
        MapStream {
            stream: self,
            func: func,
            marker: PhantomData::<O>,
        }
    }
}

impl<S> MappableStream for S where S: Stream
{}
