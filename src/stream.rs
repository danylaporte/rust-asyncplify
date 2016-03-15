use std::prelude::v1::*;
use std::marker::PhantomData;
use std::ops::Add;
use consumer::*;
use fold::*;
use subscription::*;

pub trait Stream {
    type Item;

    fn consume<C>(self, C) where C: Consumer<Item = Self::Item>;

    fn sum<O>(self) -> FoldStream<Self, fn(&O, Self::Item) -> O, O>
        where Self: Sized,
              O: Add<Self::Item, Output = O> + Default + Copy
    {
        fn adder<O, I>(v: &O, i: I) -> O
            where O: Add<I, Output = O> + Copy
        {
            *v + i
        }

        self.fold(Default::default(), adder)
    }
}
