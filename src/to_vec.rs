use consumer::*;
use producer::*;
use std::mem::replace;
use std::rc::Rc;
use stream::*;

struct ToVecState<C, F, T> {
    consumer: C,
    splitter: F,
    vec: Vec<T>,
}

pub struct ToVec<S, F> {
    stream: S,
    splitter: F,
}

impl<C, F, T> Consumer<T> for ToVecState<C, F, T>
    where C: Consumer<Vec<T>>,
          F: FnMut(&Vec<T>) -> bool
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

    fn end(mut self) {
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
    fn consume<C: Consumer<Vec<T>>>(self, consumer: C) {
        self.stream.consume(ToVecState {
            consumer: consumer,
            splitter: self.splitter,
            vec: Vec::new(),
        });
    }
}

pub trait ToVecStream<T>: Stream<T> {
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
