use std::marker::PhantomData;
use std::rc::Rc;
use consumer::*;
use producer::*;
use stream::*;

struct FoldState<C, F, I, O> {
    consumer: C,
    func: F,
    value: O,
    marker: PhantomData<I>,
}

impl<C, F, I, O> Consumer for FoldState<C, F, I, O>
    where C: Consumer<Item = O>,
          F: Fn(&O, I) -> O
{
    type Item = I;

    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: Self::Item) {
        self.value = (self.func)(&self.value, item);
    }

    fn end(mut self) {
        self.consumer.emit(self.value);
        self.consumer.end();
    }
}

pub struct FoldStream<S, F, O> {
    stream: S,
    func: F,
    initial: O,
}

impl<S, F, O> Stream for FoldStream<S, F, O>
    where S: Stream,
          F: Fn(&O, <S as Stream>::Item) -> O
{
    type Item = O;

    fn consume<C>(self, consumer: C)
        where C: Consumer<Item = Self::Item>
    {
        self.stream.consume(FoldState {
            consumer: consumer,
            func: self.func,
            value: self.initial,
            marker: PhantomData::<S::Item>,
        });
    }
}

pub trait FoldableStream: Stream {
    fn fold<O, F>(self, initial: O, func: F) -> FoldStream<Self, F, O>
        where Self: Sized,
              F: Fn(&O, Self::Item) -> O
    {
        FoldStream {
            stream: self,
            initial: initial,
            func: func,
        }
    }
}

impl<S> FoldableStream for S where S: Stream
{}
