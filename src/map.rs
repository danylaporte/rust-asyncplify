use consumer::*;
use stream::*;

struct MapState<C, F> {
    consumer: C,
    func: F,
}

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct Map<S, F> {
    stream: S,
    func: F,
}

impl<C, F, I, O> Consumer<I> for MapState<C, F>
    where F: FnMut(I) -> O,
          C: Consumer<O>
{
    fn emit(&mut self, item: I) -> bool {
        self.consumer.emit((self.func)(item))
    }
}

impl<S, F> Map<S, F> {
    pub fn new(stream: S, func: F) -> Self {
        Map {
            stream: stream,
            func: func,
        }
    }
}

impl<S, F, O> Stream for Map<S, F>
    where S: Stream,
          F: FnMut(S::Item) -> O
{
    type Item = F::Output;

    fn consume<C>(self, consumer: C)
        where C: Consumer<Self::Item>
    {
        self.stream.consume(MapState {
            consumer: consumer,
            func: self.func,
        });
    }
}
