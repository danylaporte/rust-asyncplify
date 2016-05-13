use producer::*;
use std::rc::Rc;

pub trait Consumer<T> {
    fn init(&mut self, Rc<Producer>);
    fn emit(&mut self, T);
}