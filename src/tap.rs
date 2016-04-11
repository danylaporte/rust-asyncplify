use std::rc::Rc;
use consumer::*;
use producer::*;
use stream::*;

struct TapState<'a, F, T: 'a> {
    consumer: &'a mut Consumer<T>,
    func: F,
}

pub struct Tap<S, F> {
    stream: S,
    func: F,
}

impl<'a, F: FnMut(&T), T> Consumer<T> for TapState<'a, F, T> {
    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: T) {
        (self.func)(&item);
        self.consumer.emit(item);
    }

    fn end(&mut self) {
        self.consumer.end();
    }
}

impl<S, F, T> Stream<T> for Tap<S, F>
    where S: Stream<T>,
          F: FnMut(&T)
{
    fn consume(self, consumer: &mut Consumer<T>) {
        self.stream.consume(&mut TapState {
            consumer: consumer,
            func: self.func,
        });
    }
}

pub trait TappableStream<T> : Stream<T> {
    fn tap<F>(self, func: F) -> Tap<Self, F>
        where F: FnMut(&T),
              Self: Sized
    {
        Tap {
            stream: self,
            func: func,
        }
    }
}

impl<S, T> TappableStream<T> for S where S: Stream<T> {}
