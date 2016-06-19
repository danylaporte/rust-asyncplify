use consumer::*;
use std::cmp::Ord;
use std::collections::*;

impl<'a, T> Consumer<T> for &'a mut BinaryHeap<T>
    where T: Ord
{
    fn emit(&mut self, item: T) -> bool {
        self.push(item);
        true
    }
}

impl<'a, T> Consumer<T> for &'a mut LinkedList<T> {
    fn emit(&mut self, item: T) -> bool {
        self.push_front(item);
        true
    }
}

impl<'a, T> Consumer<T> for &'a mut Vec<T> {
    fn emit(&mut self, item: T) -> bool {
        self.push(item);
        true
    }
}

impl<'a, T> Consumer<T> for &'a mut VecDeque<T> {
    fn emit(&mut self, item: T) -> bool {
        self.push_front(item);
        true
    }
}
