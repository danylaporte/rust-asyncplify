use producer::*;
use std::rc::Rc;

pub trait Consumer<T> {
    fn init(&mut self, Rc<Producer>);
    fn emit(&mut self, T);
    fn end(self);
}

pub trait ConsumerBox<T> {
    fn init(&mut self, Rc<Producer>);
    fn emit(&mut self, T);
    fn end(self: Box<Self>);
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