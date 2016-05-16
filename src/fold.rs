use consumer::*;
use std::marker::PhantomData;
use std::mem::replace;
use stream::*;

struct FoldState<C, F, I, O>
    where C: Consumer<O>
{
    consumer: C,
    func: F,
    marker_i: PhantomData<I>,
    value: Option<O>,
}

impl<C, F, I, O> Consumer<I> for FoldState<C, F, I, O>
    where C: Consumer<O>,
          F: FnMut(O, I) -> O
{
    fn emit(&mut self, item: I) -> bool {
        let v = replace(&mut self.value, None).unwrap();
        self.value = Some((self.func)(v, item));
        true
    }
}

impl<C, F, I, O> Drop for FoldState<C, F, I, O>
    where C: Consumer<O>
{
    fn drop(&mut self) {
        if let Some(v) = replace(&mut self.value, None) {
            self.consumer.emit(v);
        }
    }
}

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct Fold<S, I, F, O> {
    func: F,
    initial: O,
    marker_i: PhantomData<I>,
    stream: S,
}

impl<S, I, F, O> Stream<O> for Fold<S, I, F, O>
    where S: Stream<I>,
          F: FnMut(O, I) -> O
{
    fn consume<C: Consumer<O>>(self, consumer: C) {
        self.stream.consume(FoldState {
            consumer: consumer,
            func: self.func,
            value: Some(self.initial),
            marker_i: PhantomData::<I>,
        });
    }
}

pub trait FoldableStream<I>: Stream<I> {
    fn fold<O, F>(self, initial: O, func: F) -> Fold<Self, I, F, O>
        where Self: Sized,
              F: FnMut(O, I) -> O
    {
        Fold {
            stream: self,
            initial: initial,
            func: func,
            marker_i: PhantomData::<I>,
        }
    }
}

impl<T, S> FoldableStream<T> for S where S: Stream<T> {}
