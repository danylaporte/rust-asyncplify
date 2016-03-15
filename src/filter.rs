use std::marker::PhantomData;
use std::rc::Rc;
use consumer::*;
use producer::*;
use stream::*;

struct FilterState<C, F, T> {
    consumer: C,
    func: F,
    marker: PhantomData<T>,
}

impl<C, F, T> Consumer for FilterState<C, F, T>
    where C: Consumer<Item = T>,
          F: Fn(&T) -> bool
{
    type Item = T;

    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: Self::Item) {
        if (self.func)(&item) {
            self.consumer.emit(item);
        }
    }

    fn end(self) {
        self.consumer.end();
    }
}

pub struct FilterStream<S, F> {
    stream: S,
    func: F,
}

impl<S, F> Stream for FilterStream<S, F>
    where S: Stream,
          F: Fn(&<S as Stream>::Item) -> bool
{
    type Item = S::Item;

    fn consume<C>(self, consumer: C)
        where C: Consumer<Item = Self::Item>
    {
        self.stream.consume(FilterState {
            consumer: consumer,
            func: self.func,
            marker: PhantomData::<S::Item>,
        });
    }
}

pub trait FilterableStream : Stream {
    fn filter<F>(self, func: F) -> FilterStream<Self, F>
        where Self: Sized,
              F: Fn(&Self::Item) -> bool
    {
        FilterStream {
            stream: self,
            func: func,
        }
    }
}

impl<S> FilterableStream for S where S: Stream
{}
