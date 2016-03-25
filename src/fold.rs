use std::marker::PhantomData;
use std::mem::replace;
use std::ops::Add;
use std::rc::Rc;
use consumer::*;
use producer::*;
use stream::*;

struct FoldState<C, F, I, O> {
    consumer: C,
    func: F,
    value: Option<O>,
    marker: PhantomData<I>,
}

impl<C, F, I, O> Consumer for FoldState<C, F, I, O>
    where C: Consumer<Item = O>,
          F: FnMut(O, I) -> O
{
    type Item = I;

    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: Self::Item) {
        let v = replace(&mut self.value, None).unwrap();
        self.value = Some((self.func)(v, item));
    }

    fn end(&mut self) {
        let v = replace(&mut self.value, None).unwrap();
        self.consumer.emit(v);
        self.consumer.end();
    }
}

pub struct Fold<S, F, O> {
    stream: S,
    func: F,
    initial: O,
}

impl<S, F, O> Stream for Fold<S, F, O>
    where S: Stream,
          F: FnMut(O, <S as Stream>::Item) -> O
{
    type Item = O;

    fn consume<C>(self, consumer: C)
        where C: Consumer<Item = Self::Item>
    {
        self.stream.consume(FoldState {
            consumer: consumer,
            func: self.func,
            value: Some(self.initial),
            marker: PhantomData::<S::Item>,
        });
    }
}

pub trait FoldableStream: Stream {
    fn fold<O, F>(self, initial: O, func: F) -> Fold<Self, F, O>
        where Self: Sized,
              F: FnMut(O, Self::Item) -> O
    {
        Fold {
            stream: self,
            initial: initial,
            func: func,
        }
    }

    fn sum<O>(self) -> Fold<Self, fn(O, Self::Item) -> O, O>
        where Self: Sized,
              O: Add<Self::Item, Output = O> + Default + Copy
    {
        fn adder<O, I>(v: O, i: I) -> O
            where O: Add<I, Output = O> + Copy
        {
            v + i
        }

        self.fold(Default::default(), adder)
    }
}

impl<S> FoldableStream for S where S: Stream
{}
