use consumer::*;
use std::mem::replace;
use stream::*;

struct ScanState<C, F, O>
    where C: Consumer<O>
{
    consumer: C,
    func: F,
    value: Option<O>,
}

impl<C, F, I, O> Consumer<I> for ScanState<C, F, O>
    where C: Consumer<O>,
          F: FnMut(O, I) -> O,
          O: Clone
{
    fn emit(&mut self, item: I) -> bool {
        let v = replace(&mut self.value, None).unwrap();
        let r = (self.func)(v, item);
        self.value = Some(r.clone());
        self.consumer.emit(r)
    }
}

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct Scan<S, F, O> {
    func: F,
    initial: O,
    stream: S,
}

impl<S, F, O> Scan<S, F, O> {
    pub fn new(stream: S, initial: O, func: F) -> Self {
        Scan {
            func: func,
            initial: initial,
            stream: stream,
        }
    }
}

impl<S, F, O> Stream for Scan<S, F, O>
    where S: Stream,
          F: FnMut(O, S::Item) -> O,
          O: Clone
{
    type Item = O;

    fn consume<C>(self, consumer: C)
        where C: Consumer<O>
    {
        self.stream.consume(ScanState {
            consumer: consumer,
            func: self.func,
            value: Some(self.initial),
        });
    }
}
