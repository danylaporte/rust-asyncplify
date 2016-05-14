use consumer::*;
use producer::*;
use std::marker::PhantomData;
use std::rc::Rc;
use stream::*;

struct MapState<C, F, I, O> {
    consumer: C,
    func: F,
    marker_i: PhantomData<I>,
    marker_o: PhantomData<O>,
}

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct Map<S, F, I, O> {
    stream: S,
    func: F,
    marker_i: PhantomData<I>,
    marker_o: PhantomData<O>,
}

impl<C, F, I, O> Consumer<I> for MapState<C, F, I, O>
    where C: Consumer<O>,
          F: FnMut(I) -> O
{
    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: I) {
        self.consumer.emit((self.func)(item));
    }
}

impl<S, F, I, O> Stream<O> for Map<S, F, I, O>
    where S: Stream<I>,
          F: FnMut(I) -> O
{
    fn consume<C: Consumer<O>>(self, consumer: C) {
        self.stream.consume(MapState {
            consumer: consumer,
            func: self.func,
            marker_i: PhantomData::<I>,
            marker_o: PhantomData::<O>,
        });
    }
}

pub trait MappableStream<I>: Stream<I> {
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