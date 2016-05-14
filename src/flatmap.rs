use consumer::*;
use producer::*;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use stream::*;

struct FlatmapState<C, F, I, O, S>
    where C: Consumer<O>
{
    child: Rc<Child<C, O>>,
    func: F,
    marker_i: PhantomData<I>,
    marker_s: PhantomData<S>,
}

impl<C, F, I, O, S> Consumer<I> for FlatmapState<C, F, I, O, S>
    where C: Consumer<O>,
          F: FnMut(I) -> S,
          S: Stream<O>
{
    fn init(&mut self, producer: Rc<Producer>) {
        if let Some(ref cell) = self.child.consumer {
            cell.borrow_mut().init(producer);
        }
    }

    fn emit(&mut self, item: I) {
        let stream = (self.func)(item);
        stream.consume(self.child.clone());
    }
}

struct Child<C, O>
    where C: Consumer<O>
{
    consumer: Option<RefCell<C>>,
    marker_o: PhantomData<O>,
}

impl<C, O> Consumer<O> for Rc<Child<C, O>>
    where C: Consumer<O>
{
    fn init(&mut self, _: Rc<Producer>) {}

    fn emit(&mut self, item: O) {
        if let Some(ref cell) = self.consumer {
            cell.borrow_mut().emit(item);
        }
    }
}

pub struct Flatmap<S, F, I, SO, O> {
    func: F,
    marker_i: PhantomData<I>,
    marker_o: PhantomData<O>,
    marker_so: PhantomData<SO>,
    stream: S,
}

impl<S, I, F, SO, O> Stream<O> for Flatmap<S, F, I, SO, O>
    where S: Stream<I>,
          F: FnMut(I) -> SO,
          SO: Stream<O>
{
    fn consume<C: Consumer<O>>(self, consumer: C) {
        self.stream.consume(FlatmapState {
            child: Rc::new(Child {
                consumer: Some(RefCell::new(consumer)),
                marker_o: PhantomData::<O>,
            }),
            func: self.func,
            marker_i: PhantomData::<I>,
            marker_s: PhantomData::<SO>,
        });
    }
}

pub trait FlatmapStream<I>: Stream<I> {
    fn flatmap<F, SO, O>(self, func: F) -> Flatmap<Self, F, I, SO, O>
        where Self: Sized,
              F: FnMut(I) -> SO,
              SO: Stream<O>
    {
        Flatmap {
            func: func,
            marker_i: PhantomData::<I>,
            marker_o: PhantomData::<O>,
            marker_so: PhantomData::<SO>,
            stream: self,
        }
    }
}

impl<S, T> FlatmapStream<T> for S where S: Stream<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use iter::*;
    use stream::*;
    use subscription::*;
    use value::*;

    #[test]
    fn it_works() {
        let mut count = 0i32;
        let mut value = 0i32;

        (0..4i32)
            .to_stream()
            .flatmap(|v| Value::new(v + 10))
            .inspect(|v| {
                count += 1;
                value += *v;
            })
            .subscribe();

        assert!(count == 4, "count = {}", count);
        assert!(value == 10 + 11 + 12 + 13, "value = {}", value);
    }
}
