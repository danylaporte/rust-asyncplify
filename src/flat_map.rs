use consumer::*;
use std::cell::RefCell;
use std::rc::Rc;
use stream::*;

struct FlatmapState<C, F> {
    child: Rc<RefCell<Option<C>>>,
    func: F,
}

impl<C, F, I, S> Consumer<I> for FlatmapState<C, F>
    where F: FnMut(I) -> S,
          C: Consumer<S::Item>,
          S: Stream
{
    fn emit(&mut self, item: I) -> bool {
        if self.child.borrow().is_none() {
            return false;
        }

        let stream = (self.func)(item);
        stream.consume(self.child.clone());
        self.child.borrow().is_some()
    }
}

impl<C, T> Consumer<T> for Rc<RefCell<Option<C>>>
    where C: Consumer<T>
{
    fn emit(&mut self, item: T) -> bool {
        let mut consumer = self.borrow_mut();

        if let Some(ref mut consumer) = *consumer {
            if consumer.emit(item) {
                return true;
            }
        }

        *consumer = None;
        false
    }
}

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct Flatmap<S, F> {
    func: F,
    stream: S,
}

impl<S, F> Flatmap<S, F> {
    pub fn new(stream: S, func: F) -> Self {
        Flatmap {
            func: func,
            stream: stream,
        }
    }
}

impl<S, F, SO> Stream for Flatmap<S, F>
    where S: Stream,
          F: FnMut(S::Item) -> SO,
          SO: Stream
{
    type Item = SO::Item;

    fn consume<C>(self, consumer: C)
        where C: Consumer<Self::Item>
    {
        self.stream.consume(FlatmapState {
            child: Rc::new(RefCell::new(Some(consumer))),
            func: self.func,
        });
    }
}