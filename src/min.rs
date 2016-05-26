use consumer::*;
use std::cmp::PartialOrd;
use std::mem::replace;
use stream::*;

struct MinState<C, T>
    where C: Consumer<T>,
          T: PartialOrd
{
    consumer: C,
    value: Option<T>,
}

impl<C, T> Consumer<T> for MinState<C, T>
    where C: Consumer<T>,
          T: PartialOrd
{
    fn emit(&mut self, item: T) -> bool {
        
        if let Some(current) = self.value.take() {
            self.value = Some(if current < item { current } else { item });
        } else {
            self.value = Some(item);
        }

        true
    }
}

impl<C, T> Drop for MinState<C, T>
    where C: Consumer<T>,
          T: PartialOrd
{
    fn drop(&mut self) {
        if let Some(value) = replace(&mut self.value, None) {
            self.consumer.emit(value);
        }
    }
}

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct Min<S> {
    stream: S,
}

impl<S, T> Stream<T> for Min<S>
    where S: Stream<T>,
          T: PartialOrd
{
    fn consume<C: Consumer<T>>(self, consumer: C) {
        self.stream.consume(MinState {
            consumer: consumer,
            value: None,
        });
    }
}

impl<S> Min<S> {
    pub fn new(stream: S) -> Self {
        Min { stream: stream }
    }
}