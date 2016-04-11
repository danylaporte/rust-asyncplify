use std::rc::Rc;
use consumer::*;
use producer::*;
use stream::*;
use std::mem::replace;

struct ToVecState<'a, F, T: 'a> {
    consumer: &'a mut Consumer<Vec<T>>,
    splitter: F,
    vec: Vec<T>,
}

pub struct ToVec<S, F> {
    stream: S,
    splitter: F,
}

impl<'a, F, T> Consumer<T> for ToVecState<'a, F, T>
    where F: FnMut(&Vec<T>) -> bool
{
    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: T) {
        self.vec.push(item);

        if (self.splitter)(&self.vec) {
            let vec = replace(&mut self.vec, Vec::new());
            self.consumer.emit(vec);
        }
    }

    fn end(&mut self) {
        if self.vec.len() != 0 {
            let vec = replace(&mut self.vec, Vec::new());
            self.consumer.emit(vec);
        }

        self.consumer.end();
    }
}

impl<S, F, T> Stream<Vec<T>> for ToVec<S, F>
    where S: Stream<T>,
          F: FnMut(&Vec<T>) -> bool
{
    fn consume(self, consumer: &mut Consumer<Vec<T>>) {
        self.stream.consume(&mut ToVecState {
            consumer: consumer,
            splitter: self.splitter,
            vec: Vec::new(),
        });
    }
}

pub trait ToVecStream<T> : Stream<T> {
    fn to_vec<F>(self, splitter: F) -> ToVec<Self, F>
        where F: FnMut(&Vec<T>),
              Self: Sized
    {
        ToVec {
            stream: self,
            splitter: splitter,
        }
    }
}

impl<S, T> ToVecStream<T> for S where S: Stream<T> {}
