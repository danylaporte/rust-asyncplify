use consumer::*;
use producer::*;
use std::marker::PhantomData;
use std::mem::replace;
use std::ops::Add;
use std::rc::Rc;
use stream::*;

struct FoldState<C, F, I, O> {
    consumer: C,
    func: F,
    value: Option<O>,
    marker_i: PhantomData<I>,
}

impl<C, F, I, O> Consumer<I> for FoldState<C, F, I, O>
    where C: Consumer<O>,
          F: FnMut(O, I) -> O
{
    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: I) {
        let v = replace(&mut self.value, None).unwrap();
        self.value = Some((self.func)(v, item));
    }

    fn end(mut self) {
        let v = replace(&mut self.value, None).unwrap();
        self.consumer.emit(v);
        self.consumer.end();
    }
}

pub struct Fold<S, I, F, O> {
    stream: S,
    func: F,
    initial: O,
    marker_i: PhantomData<I>,
}

impl<S, I, F, O> Stream<O> for Fold<S, I, F, O>
    where S: Stream<I>,
          F: FnMut(O, I) -> O
{
    fn consume<C: Consumer<O>>(self, consumer: C) {
        self.stream.consume(FoldState {
            consumer: consumer,
            func: self.func,
            value: Some(self.initial),
            marker_i: PhantomData::<I>,
        });
    }
}

pub trait FoldableStream<I>: Stream<I> {
    fn fold<O, F>(self, initial: O, func: F) -> Fold<Self, I, F, O>
        where Self: Sized,
              F: FnMut(O, I) -> O
    {
        Fold {
            stream: self,
            initial: initial,
            func: func,
            marker_i: PhantomData::<I>,
        }
    }

    fn sum<O>(self) -> Fold<Self, I, fn(O, I) -> O, O>
        where Self: Sized,
              O: Add<I, Output = O> + Default + Copy
    {
        fn adder<O, I>(v: O, i: I) -> O
            where O: Add<I, Output = O> + Copy
        {
            v + i
        }

        self.fold(Default::default(), adder)
    }
}

impl<T, S> FoldableStream<T> for S where S: Stream<T> {}
