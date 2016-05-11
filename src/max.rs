use consumer::*;
use producer::*;
use std::cmp::PartialOrd;
use std::mem::replace;
use std::rc::Rc;
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
    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: T) {
        if let Some(ref value) = self.value {
            if value >= &item {
                return;
            }
        }

        self.value = Some(item);
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