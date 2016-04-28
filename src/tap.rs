use consumer::*;
use producer::*;
use std::marker::PhantomData;
use std::rc::Rc;
use stream::*;

struct TapState<C, F, T> {
    consumer: C,
    func: F,
    marker_t: PhantomData<T>,
}

pub struct Tap<S, F> {
    stream: S,
    func: F,
}

impl<C, F: FnMut(&mut T), T> Consumer<T> for TapState<C, F, T>
    where C: Consumer<T>,
          F: FnMut(&mut T)
{
    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, mut item: T) {
        (self.func)(&mut item);
        self.consumer.emit(item);
    }
}

impl<C, F: FnMut(&T), T> ConsumerRef<T> for TapState<C, F, T>
    where C: ConsumerRef<T>,
          F: FnMut(&T)
{
    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: &T) {
        (self.func)(item);
        self.consumer.emit(item);
    }
}

impl<C, F: FnMut(&mut T), T> ConsumerRefMut<T> for TapState<C, F, T>
    where C: ConsumerRefMut<T>,
          F: FnMut(&mut T)
{
    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: &mut T) {
        (self.func)(item);
        self.consumer.emit(item);
    }
}

impl<S, F, T> Stream<T> for Tap<S, F>
    where S: Stream<T>,
          F: FnMut(&mut T)
{
    fn consume<C: Consumer<T>>(self, consumer: C) {
        self.stream.consume(TapState {
            consumer: consumer,
            func: self.func,
            marker_t: PhantomData::<T>,
        });
    }
}

impl<S, F, T> StreamRef<T> for Tap<S, F>
    where S: StreamRef<T>,
          F: FnMut(&T)
{
    fn consume<C: ConsumerRef<T>>(self, consumer: C) {
        self.stream.consume(TapState {
            consumer: consumer,
            func: self.func,
            marker_t: PhantomData::<T>,
        });
    }
}

impl<S, F, T> StreamRefMut<T> for Tap<S, F>
    where S: StreamRefMut<T>,
          F: FnMut(&mut T)
{
    fn consume<C: ConsumerRefMut<T>>(self, consumer: C) {
        self.stream.consume(TapState {
            consumer: consumer,
            func: self.func,
            marker_t: PhantomData::<T>,
        });
    }
}

pub trait TappableStream<T>: Stream<T> {
    fn tap<F>(self, func: F) -> Tap<Self, F>
        where F: FnMut(&mut T),
              Self: Sized
    {
        Tap {
            stream: self,
            func: func,
        }
    }
}

pub trait TappableStreamRef<T>: StreamRef<T> {
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

pub trait TappableStreamRefMut<T>: StreamRefMut<T> {
    fn tap<F>(self, func: F) -> Tap<Self, F>
        where F: FnMut(&mut T),
              Self: Sized
    {
        Tap {
            stream: self,
            func: func,
        }
    }
}

impl<S, T> TappableStream<T> for S where S: Stream<T> {}
impl<S, T> TappableStreamRef<T> for S where S: StreamRef<T> {}
impl<S, T> TappableStreamRefMut<T> for S where S: StreamRefMut<T> {}
