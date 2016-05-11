use consumer::*;
use producer::*;
use std::rc::Rc;
use stream::*;

struct FilterState<C, F> {
    consumer: C,
    predicate: F,
}

impl<C, F, T> Consumer<T> for FilterState<C, F>
    where C: Consumer<T>,
          F: FnMut(&mut T) -> bool
{
    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, mut item: T) {
        if (self.predicate)(&mut item) {
            self.consumer.emit(item);
        }
    }
}

/// Describe a filter for a `stream`.
pub struct Filter<S, F> {
    stream: S,
    predicate: F,
}

impl<S, F, T> Stream<T> for Filter<S, F>
    where S: Stream<T>,
          F: FnMut(&mut T) -> bool
{
    fn consume<C: Consumer<T>>(self, consumer: C) {
        self.stream.consume(FilterState {
            consumer: consumer,
            predicate: self.predicate,
        });
    }
}

impl<S, F> Filter<S, F> {
    pub fn new(stream: S, predicate: F) -> Self {
        Filter {
            predicate: predicate,
            stream: stream,
        }
    }
}