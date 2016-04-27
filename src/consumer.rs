use producer::*;
use std::convert::From;
use std::marker::PhantomData;
use std::rc::Rc;

pub trait BoxedConsumer<T> {
    fn init(&mut self, Rc<Producer>);
    fn emit(&mut self, T);
    fn end(self: Box<Self>);
}

pub trait Consumer<T> {
    fn init(&mut self, Rc<Producer>);
    fn emit(&mut self, T);
    fn end(self);
}

pub trait ConsumerRef<T> {
    fn init(&mut self, Rc<Producer>);
    fn emit(&mut self, &T);
    fn end(self);
}

pub trait ConsumerRefMut<T> {
    fn init(&mut self, Rc<Producer>);
    fn emit(&mut self, &mut T);
    fn end(self);
}

pub struct ConsumerBox<C, T> {
    consumer: C,
    marker_t: PhantomData<T>,
}

impl<C, T> From<C> for ConsumerBox<C, T>
    where C: Consumer<T>
{
    fn from(consumer: C) -> Self {
        ConsumerBox {
            consumer: consumer,
            marker_t: PhantomData::<T>,
        }
    }
}

impl<C, T> BoxedConsumer<T> for ConsumerBox<C, T>
    where C: Consumer<T>
{
    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: T) {
        self.consumer.emit(item);
    }

    fn end(self: Box<Self>) {
        self.consumer.end();
    }
}