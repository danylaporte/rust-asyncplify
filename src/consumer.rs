use std::rc::Rc;
use producer::*;

pub trait Consumer<T> {
    fn init(&mut self, Rc<Producer>);
    fn emit(&mut self, T);
    fn end(&mut self);
}

pub trait ConsumerRef<T> {
    fn init(&mut self, Rc<Producer>);
    fn emit<'a>(&mut self, &'a T);
    fn end(&mut self);
}

pub trait ConsumerRefMut<T> {
    fn init(&mut self, Rc<Producer>);
    fn emit<'a>(&mut self, &'a mut T);
    fn end(&mut self);
}