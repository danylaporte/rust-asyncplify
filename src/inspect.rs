use consumer::*;
use producer::*;
use std::rc::Rc;
use stream::*;

struct InspectState<C, F> {
    consumer: C,
    func: F,
}

pub struct Inspect<S, F> {
    func: F,
    stream: S,
}

impl<S, F> Inspect<S, F> {
    pub fn new(stream: S, func: F) -> Self {
        Inspect {
            func: func,
            stream: stream,
        }
    }
}

impl<C, F, T> Consumer<T> for InspectState<C, F>
    where C: Consumer<T>,
          F: FnMut(&mut T)
{
    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, mut item: T) {
        (self.func)(&mut item);
        self.consumer.emit(item);
    }
}

impl<S, F, T> Stream<T> for Inspect<S, F>
    where S: Stream<T>,
          F: FnMut(&mut T)
{
    fn consume<C: Consumer<T>>(self, consumer: C) {
        self.stream.consume(InspectState {
            consumer: consumer,
            func: self.func,
        });
    }
}