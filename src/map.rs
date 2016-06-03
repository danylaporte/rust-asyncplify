use consumer::*;
use std::marker::PhantomData;
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
    fn emit(&mut self, item: I) -> bool {
        self.consumer.emit((self.func)(item))
    }
}

impl<S, F, I, O> Map<S, F, I, O> {
    pub fn new(stream: S, func: F) -> Self {
        Map {
            stream: stream,
            func: func,
            marker_i: PhantomData::<I>,
            marker_o: PhantomData::<O>,
        }
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
