use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use consumer::*;
use producer::*;
use stream::*;

struct FlatmapState<C, F, I, SO, O> {
    func: F,
    marker_i: PhantomData<I>,
    marker_o: PhantomData<O>,
    marker_so: PhantomData<SO>,
    shared: Rc<RefCell<Shared<C, O>>>,
}

struct Shared<C, T> {
    consumer: C,
    count: usize,
    marker_t: PhantomData<T>,
    producers: Vec<(usize, Option<Rc<Producer>>)>,
}

impl<C: Consumer<T>, T> Shared<C, T> {
    fn add_producer(&mut self) -> usize {
        self.count += 1;
        self.producers.push((self.count, None));
        self.count
    }

    fn new(consumer: C) -> Self {
        let mut s = Shared {
            consumer: consumer,
            count: 0,
            producers: Vec::new(),
            marker_t: PhantomData::<T>,
        };

        s.add_producer();
        s
    }

    fn init(&mut self, id: usize, producer: Rc<Producer>) {
        if let Some(index) = self.producers.iter().position(|t| t.0 == id) {
            let id = self.producers[index].0;
            self.producers[index] = (id, Some(producer));
        }
    }

    fn end(&mut self, id: usize) {
        let mut is_closable = false;

        if let Some(index) = self.producers.iter().position(|t| t.0 == id) {
            self.producers.swap_remove(index);
            is_closable = self.producers.len() == 0;
        }

        if is_closable {
            self.consumer.end();
        }
    }
}

struct Child<C, T> {
    id: usize,
    shared: Rc<RefCell<Shared<C, T>>>,
}

impl<C: Consumer<T>, T> Consumer<T> for Child<C, T> {
    fn init(&mut self, producer: Rc<Producer>) {
        self.shared.borrow_mut().init(self.id, producer);
    }

    fn emit(&mut self, item: T) {
        self.shared.borrow_mut().consumer.emit(item);
    }

    fn end(&mut self) {
        self.shared.borrow_mut().end(self.id);
    }
}

impl<C, F, I, SO, O> Consumer<I> for FlatmapState<C, F, I, SO, O>
    where C: Consumer<O> + 'static,
          F: FnMut(I) -> SO,
          SO: Stream<O>,
          O: 'static,
{
    fn init(&mut self, producer: Rc<Producer>) {

        let cloned_share = self.shared.clone();
        let mut shared = self.shared.borrow_mut();

        shared.init(1, producer);

        let rc = Rc::new(Producer::from_func(Box::new(move |_| {
            let ref mut producers = cloned_share.borrow_mut().producers;

            for t in producers.iter() {
                if let Some(ref p) = t.1 {
                    p.close();
                }
            }

            producers.clear();
        })));

        shared.consumer.init(rc);
    }

    fn emit(&mut self, item: I) {
        let id = self.shared.borrow_mut().add_producer();

        (self.func)(item).consume(Child {
            id: id,
            shared: self.shared.clone(),
        });
    }

    fn end(&mut self) {
        self.shared.borrow_mut().end(1);
    }
}

pub struct Flatmap<S, F, I, SO, O> {
    stream: S,
    func: F,
    marker_i: PhantomData<I>,
    marker_o: PhantomData<O>,
    marker_so: PhantomData<SO>,
}

impl<S, I, F, SO, O> Stream<O> for Flatmap<S, F, I, SO, O>
    where S: Stream<I>,
          F: FnMut(I) -> SO,
          SO: Stream<O>,
          O: 'static
{
    fn consume<C>(self, consumer: C)
        where C: Consumer<O> + 'static
    {
        self.stream.consume(FlatmapState {
            func: self.func,
            marker_i: PhantomData::<I>,
            marker_o: PhantomData::<O>,
            marker_so: self.marker_so,
            shared: Rc::new(RefCell::new(Shared::new(consumer))),
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
            stream: self,
            func: func,
            marker_i: PhantomData::<I>,
            marker_so: PhantomData::<SO>,
            marker_o: PhantomData::<O>,
        }
    }
}

impl<S, T> FlatmapStream<T> for S where S: Stream<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use iter::*;
    use subscription::*;
    use tap::*;
    use value::*;

    #[test]
    fn it_works() {
        let mut count = 0i32;
        let mut value = 0i32;

        (0..4i32)
            .to_stream()
            .flatmap(|v| Value::new(v + 10))
            .tap(|v| {
                count += 1;
                value += *v;
            })
            .subscribe();

        assert!(count == 4, "count = {}", count);
        assert!(value == 10 + 11 + 12 + 13, "value = {}", value);
    }
}
