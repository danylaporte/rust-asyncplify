use consumer::*;
use std::cmp::Ord;
use stream::*;

pub struct Sort<S> {
    stream: S,
}

impl<S> Sort<S> {
    pub fn new(stream: S) -> Self {
        Sort { stream: stream }
    }
}

struct SortState<C, T>
    where C: Consumer<T>,
          T: Ord
{
    consumer: C,
    vec: Vec<T>,
}

impl<C, T> Consumer<T> for SortState<C, T>
    where C: Consumer<T>,
          T: Ord
{
    fn emit(&mut self, item: T) -> bool {
        self.vec.push(item);
        true
    }
}

impl<C, T> Drop for SortState<C, T>
    where C: Consumer<T>,
          T: Ord
{
    fn drop(&mut self) {
        self.vec.sort();

        for i in self.vec.drain(..) {
            if !self.consumer.emit(i) {
                break;
            }
        }
    }
}

impl<S, T> Stream<T> for Sort<S>
    where S: Stream<T>,
          T: Ord
{
    fn consume<C>(self, consumer: C)
        where C: Consumer<T>
    {
        self.stream.consume(SortState {
            consumer: consumer,
            vec: Vec::new(),
        });
    }
}