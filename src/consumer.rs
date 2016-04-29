use producer::*;
use std::rc::Rc;

pub trait Consumer<T> {
    fn init(&mut self, Rc<Producer>);
    fn emit(&mut self, T);
}

pub trait ConsumerRef<T> {
    fn init(&mut self, Rc<Producer>);
    fn emit(&mut self, &T);
}

pub trait ConsumerRefMut<T> {
    fn init(&mut self, Rc<Producer>);
    fn emit(&mut self, &mut T);
}