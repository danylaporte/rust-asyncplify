use std::marker::PhantomData;

use std::rc::Rc;
use consumer::*;
use producer::*;
use stream::*;

struct MapState<'a, F, I, O: 'a> {
    consumer: &'a mut Consumer<O>,
    func: F,
    marker_i: PhantomData<I>,
}

pub struct Map<S, F, I, O> {
    stream: S,
    func: F,
    marker_i: PhantomData<I>,
    marker_o: PhantomData<O>,
}

impl<'a, F: FnMut(I) -> O, I, O> Consumer<I> for MapState<'a, F, I, O> {
    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: I) {
        self.consumer.emit((self.func)(item));
    }

    fn end(&mut self) {
        self.consumer.end();
    }
}

impl<S: Stream<I>, F: FnMut(I) -> O, I, O> Stream<O> for Map<S, F, I, O> {
    fn consume(self, consumer: &mut Consumer<O>) {
        self.stream.consume(&mut MapState {
            consumer: consumer,
            func: self.func,
            marker_i: PhantomData::<I>,
        });
    }
}

pub trait MappableStream<I> : Stream<I> {
    fn map<O, F>(self, func: F) -> Map<Self, F, I, O>
        where Self: Sized,
              F: FnMut(I) -> O
    {
        Map {
            stream: self,
            func: func,
            marker_i: PhantomData::<I>,
            marker_o: PhantomData::<O>,
        }
    }
}

impl<S, T> MappableStream<T> for S where S: Stream<T> {}