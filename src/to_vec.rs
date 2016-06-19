use consumer::*;
use std::mem::replace;
use stream::*;

struct ToVecState<C, F, T>
    where C: Consumer<Vec<T>>
{
    consumer: C,
    splitter: F,
    vec: Vec<T>,
}

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct ToVec<S, F> {
    stream: S,
    splitter: F,
}

impl<S, F> ToVec<S, F> {
    pub fn new(stream: S, splitter: F) -> Self {
        ToVec {
            stream: stream,
            splitter: splitter,
        }
    }
}

impl<C, F, T> Drop for ToVecState<C, F, T>
    where C: Consumer<Vec<T>>
{
    fn drop(&mut self) {
        if !self.vec.is_empty() {
            let vec = replace(&mut self.vec, Vec::new());
            self.consumer.emit(vec);
        }
    }
}

impl<C, F, T> Consumer<T> for ToVecState<C, F, T>
    where C: Consumer<Vec<T>>,
          F: FnMut(&Vec<T>) -> bool
{
    fn emit(&mut self, item: T) -> bool {
        self.vec.push(item);

        if (self.splitter)(&self.vec) {
            let vec = replace(&mut self.vec, Vec::new());
            self.consumer.emit(vec)
        } else {
            true
        }
    }
}

impl<S, F> Stream for ToVec<S, F>
    where S: Stream,
          F: FnMut(&Vec<S::Item>) -> bool
{
    type Item = Vec<S::Item>;

    fn consume<C>(self, consumer: C)
        where C: Consumer<Self::Item>
    {
        self.stream.consume(ToVecState {
            consumer: consumer,
            splitter: self.splitter,
            vec: Vec::new(),
        });
    }
}
