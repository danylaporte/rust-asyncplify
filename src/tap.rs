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

pub struct Tap<S, F> {
    stream: S,
    func: F,
}

impl<C, F, T> Consumer<T> for TapState<C, F, T>
    where C: Consumer<T>,
          F: FnMut(&T)
{
    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: T) {
        (self.func)(&item);
        self.consumer.emit(item);
    }

    fn end(&mut self) {
        self.consumer.end();
    }
}

impl<S, F, T> Stream<T> for Tap<S, F>
    where S: Stream<T>,
          F: FnMut(&T)
{
    fn consume<C: Consumer<T>>(self, consumer: C) {
        self.stream.consume(TapState {
            consumer: consumer,
            func: self.func,
            marker: PhantomData::<T>,
        });
    }
}

pub trait TappableStream<T> : Stream<T> {
    fn tap<F>(self, func: F) -> Tap<Self, F>
        where F: FnMut(&T),
              Self: Sized
    {
        Tap {
            stream: self,
            func: func,
        }
    }
}

impl<S, T> TappableStream<T> for S where S: Stream<T> {}
