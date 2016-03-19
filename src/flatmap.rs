use std::cell::RefCell;
use std::marker::PhantomData;
use std::mem;
use std::rc::Rc;
use consumer::*;
use producer::*;
use stream::*;

struct FlatmapState<C, F, I, SO> {
    func: F,
    marker_i: PhantomData<I>,
    marker_so: PhantomData<SO>,
    shared: Rc<RefCell<Shared<C>>>,
}

struct Shared<C> {
    consumer: Option<C>,
    count: usize,
    producers: Vec<(usize, Option<Rc<Producer>>)>,
}

impl<C> Shared<C> where C: Consumer
{
    fn add_producer(&mut self) -> usize {
        self.count += 1;
        self.producers.push((self.count, None));
        self.count
    }

    fn new(consumer: C) -> Self {
        let mut s = Shared {
            consumer: Some(consumer),
            count: 0,
            producers: Vec::new(),
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
            if let Some(consumer) = mem::replace(&mut self.consumer, None) {
                consumer.end();
            }
        }
    }
}

struct Child<C> {
    id: usize,
    shared: Rc<RefCell<Shared<C>>>,
}

impl<C> Consumer for Child<C> where C: Consumer
{
    type Item = C::Item;

    fn init(&mut self, producer: Rc<Producer>) {
        self.shared.borrow_mut().init(self.id, producer);
    }

    fn emit(&mut self, item: Self::Item) {
        if let Some(ref mut consumer) = self.shared.borrow_mut().consumer {
            consumer.emit(item);
        }
    }

    fn end(self) {
        self.shared.borrow_mut().end(self.id);
    }
}

impl<C, F, I, SO> Consumer for FlatmapState<C, F, I, SO>
    where C: Consumer + 'static,
          F: FnMut(I) -> SO,
          SO: Stream<Item = C::Item>
{
    type Item = I;

    fn init(&mut self, producer: Rc<Producer>) {

        let cloned_share = self.shared.clone();
        let mut shared = self.shared.borrow_mut();

        shared.init(1, producer);

        if let Some(ref mut consumer) = shared.consumer {
            let rc = Rc::new(Producer::from_func(Box::new(move |_| {
                let ref mut producers = cloned_share.borrow_mut().producers;

                for t in producers.iter() {
                    if let Some(ref p) = t.1 {
                        p.close();
                    }
                }

                producers.clear();
            })));

            consumer.init(rc);
        }
    }

    fn emit(&mut self, item: Self::Item) {
        let id = self.shared.borrow_mut().add_producer();

        (self.func)(item).consume(Child {
            id: id,
            shared: self.shared.clone(),
        });
    }

    fn end(self) {
        self.shared.borrow_mut().end(1);
    }
}

pub struct Flatmap<S, F, SO> {
    stream: S,
    func: F,
    marker_so: PhantomData<SO>,
}

impl<S, F, SO> Stream for Flatmap<S, F, SO>
    where S: Stream,
          F: FnMut(<S as Stream>::Item) -> SO,
          SO: Stream
{
    type Item = SO::Item;

    fn consume<C>(self, consumer: C)
        where C: Consumer<Item = Self::Item> + 'static
    {
        self.stream.consume(FlatmapState {
            func: self.func,
            marker_i: PhantomData::<S::Item>,
            marker_so: self.marker_so,
            shared: Rc::new(RefCell::new(Shared::new(consumer))),
        });
    }
}

pub trait FlatmapStream: Stream {
    fn flatmap<F, SO>(self, func: F) -> Flatmap<Self, F, SO>
        where Self: Sized,
              F: FnMut(Self::Item) -> SO,
              SO: Stream
    {
        Flatmap {
            stream: self,
            func: func,
            marker_so: PhantomData::<SO>,
        }
    }
}

impl<S> FlatmapStream for S where S: Stream
{}

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
