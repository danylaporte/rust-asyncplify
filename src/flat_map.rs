use consumer::*;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use stream::*;

struct FlatmapState<C, F, I, O, S> {
    child: Rc<RefCell<Option<C>>>,
    func: F,
    marker_i: PhantomData<I>,
    marker_o: PhantomData<O>,
    marker_s: PhantomData<S>,
}

impl<C, F, I, O, S> Consumer<I> for FlatmapState<C, F, I, O, S>
    where F: FnMut(I) -> S,
          C: Consumer<O>,
          S: Stream<O>
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
pub struct Flatmap<S, F, I, SO> {
    func: F,
    marker_i: PhantomData<I>,
    marker_so: PhantomData<SO>,
    stream: S,
}

impl<S, F, I, SO> Flatmap<S, F, I, SO> {
    pub fn new(stream: S, func: F) -> Self {
        Flatmap {
            func: func,
            marker_i: PhantomData::<I>,
            marker_so: PhantomData::<SO>,
            stream: stream,
        }
    }
}

impl<S, I, F, SO, O> Stream<O> for Flatmap<S, F, I, SO>
    where S: Stream<I>,
          F: FnMut(I) -> SO,
          SO: Stream<O>
{
    fn consume<C>(self, consumer: C)
        where C: Consumer<O>
    {
        self.stream.consume(FlatmapState {
            child: Rc::new(RefCell::new(Some(consumer))),
            func: self.func,
            marker_i: PhantomData::<I>,
            marker_o: PhantomData::<O>,
            marker_s: PhantomData::<SO>,
        });
    }
}