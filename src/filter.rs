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

impl<C, F, T> Consumer<T> for FilterState<C, F, T>
    where C: Consumer<T>,
          F: FnMut(&T) -> bool
{
    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: T) {
        if (self.func)(&item) {
            self.consumer.emit(item);
        }
    }

    fn end(&mut self) {
        self.consumer.end();
    }
}

pub struct Filter<S, F, T> {
    stream: S,
    func: F,
    marker: PhantomData<T>,
}

impl<S, F, T> Stream<T> for Filter<S, F, T>
    where S: Stream<T>,
          F: FnMut(&T) -> bool
{
    fn consume<C: Consumer<T>>(self, consumer: C) {
        self.stream.consume(FilterState {
            consumer: consumer,
            func: self.func,
            marker: PhantomData::<T>,
        });
    }
}

pub trait FilterableStream<T> : Stream<T> {
    fn filter<F>(self, func: F) -> Filter<Self, F, T>
        where Self: Sized,
              F: FnMut(&T) -> bool
    {
        Filter {
            stream: self,
            func: func,
            marker: PhantomData::<T>,
        }
    }
}

impl<S, T> FilterableStream<T> for S where S: Stream<T> {}
