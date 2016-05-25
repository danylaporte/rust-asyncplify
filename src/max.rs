use consumer::*;
use std::cmp::PartialOrd;
use std::mem::replace;
use stream::*;

struct MaxState<C, T>
    where C: Consumer<T>,
          T: PartialOrd
{
    consumer: C,
    value: Option<T>,
}

impl<C, T> Consumer<T> for MaxState<C, T>
    where C: Consumer<T>,
          T: PartialOrd
{
    fn emit(&mut self, item: T) -> bool {

        if let Some(current) = self.value.take() {
            self.value = Some(if current > item { current } else { item });
        } else {
            self.value = Some(item);
        }

        true
    }
}

impl<C, T> Drop for MaxState<C, T>
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
pub struct Max<S> {
    stream: S,
}

impl<S, T> Stream<T> for Max<S>
    where S: Stream<T>,
          T: PartialOrd
{
    fn consume<C: Consumer<T>>(self, consumer: C) {
        self.stream.consume(MaxState {
            consumer: consumer,
            value: None,
        });
    }
}

impl<S> Max<S> {
    pub fn new(stream: S) -> Self {
        Max { stream: stream }
    }
}