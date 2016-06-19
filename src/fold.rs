use consumer::*;
use std::mem::replace;
use stream::*;

struct FoldState<C, F, O>
    where C: Consumer<O>
{
    consumer: C,
    func: F,
    value: Option<O>,
}

impl<C, F, I, O> Consumer<I> for FoldState<C, F, O>
    where C: Consumer<O>,
          F: FnMut(O, I) -> O
{
    fn emit(&mut self, item: I) -> bool {
        let v = replace(&mut self.value, None).unwrap();
        self.value = Some((self.func)(v, item));
        true
    }
}

impl<C, F, O> Drop for FoldState<C, F, O>
    where C: Consumer<O>
{
    fn drop(&mut self) {
        if let Some(v) = replace(&mut self.value, None) {
            self.consumer.emit(v);
        }
    }
}

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct Fold<S, F, O> {
    func: F,
    initial: O,
    stream: S,
}

impl<S, F, O> Fold<S, F, O> {
    pub fn new(stream: S, initial: O, func: F) -> Self {
        Fold {
            stream: stream,
            initial: initial,
            func: func,
        }
    }
}

impl<S, F, O> Stream for Fold<S, F, O>
    where S: Stream,
          F: FnMut(O, S::Item) -> O
{
    type Item = O;

    fn consume<C>(self, consumer: C)
        where C: Consumer<Self::Item>
    {
        self.stream.consume(FoldState {
            consumer: consumer,
            func: self.func,
            value: Some(self.initial),
        });
    }
}
