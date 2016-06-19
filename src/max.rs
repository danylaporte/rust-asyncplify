use consumer::*;
use std::cmp::PartialOrd;
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
    fn emit(&mut self, mut item: T) -> bool {

        if let Some(current) = self.value.take() {
            if current > item {
                item = current;
            }
        }

        self.value = Some(item);
        true
    }
}

impl<C, T> Drop for MaxState<C, T>
    where C: Consumer<T>,
          T: PartialOrd
{
    fn drop(&mut self) {
        if let Some(value) = self.value.take() {
            self.consumer.emit(value);
        }
    }
}

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct Max<S> {
    stream: S,
}

impl<S> Stream for Max<S>
    where S: Stream,
          S::Item: PartialOrd
{
    type Item = S::Item;

    fn consume<C>(self, consumer: C)
        where C: Consumer<Self::Item>
    {
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
