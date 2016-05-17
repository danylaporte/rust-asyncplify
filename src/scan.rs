use consumer::*;
use std::marker::PhantomData;
use std::mem::replace;
use stream::*;

struct ScanState<C, F, I, O>
    where C: Consumer<O>
{
    consumer: C,
    func: F,
    marker_i: PhantomData<I>,
    value: Option<O>,
}

impl<C, F, I, O> Consumer<I> for ScanState<C, F, I, O>
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
pub struct Scan<S, I, F, O> {
    func: F,
    initial: O,
    marker_i: PhantomData<I>,
    stream: S,
}

impl<S, I, F, O> Scan<S, I, F, O> {
    pub fn new(stream: S, initial: O, func: F) -> Self {
        Scan {
            func: func,
            initial: initial,
            marker_i: PhantomData::<I>,
            stream: stream,
        }
    }
}

impl<S, I, F, O> Stream<O> for Scan<S, I, F, O>
    where S: Stream<I>,
          F: FnMut(O, I) -> O,
          O: Clone
{
    fn consume<C: Consumer<O>>(self, consumer: C) {
        self.stream.consume(ScanState {
            consumer: consumer,
            func: self.func,
            marker_i: PhantomData::<I>,
            value: Some(self.initial),
        });
    }
}
