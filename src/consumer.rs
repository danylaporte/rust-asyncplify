use std::cell::Cell;
use std::rc::Rc;
use producer::*;

pub trait Consumer {
    type Item;

    fn init(&mut self, Rc<Producer>);
    fn emit(&mut self, Self::Item);
    fn end(self);
}
