use consumer::*;
use std::cell::RefCell;
use std::mem::replace;
use std::rc::Rc;
use stream::*;

type Consumers<T> = Rc<RefCell<Vec<Box<Consumer<T>>>>>;

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct Clonable<S>
    where S: Stream,
          S::Item: Clone
{
    stream: Option<Rc<ClonableStream<S>>>,
}

impl<S> Clonable<S>
    where S: Stream,
          S::Item: Clone
{
    pub fn new(stream: S) -> Self {
        Clonable { stream: Some(Rc::new(ClonableStream::new(stream))) }
    }
}

impl<S> Stream for Clonable<S>
    where S: Stream,
          S::Item: Clone
{
    type Item = S::Item;

    fn consume<C>(self, consumer: C)
        where C: Consumer<Self::Item> + 'static
    {
        if let Some(ref stream) = self.stream {
            stream.consumers.borrow_mut().push(Box::new(consumer));
        }
    }
}

impl<S> Clone for Clonable<S>
    where S: Stream,
          S::Item: Clone
{
    fn clone(&self) -> Self {
        Clonable { stream: self.stream.clone() }
    }
}

impl<S> Drop for Clonable<S>
    where S: Stream,
          S::Item: Clone
{
    fn drop(&mut self) {
        let stream = replace(&mut self.stream, None).unwrap();

        if let Ok(clonable_stream) = Rc::try_unwrap(stream) {
            let stream = clonable_stream.stream;
            let consumers = clonable_stream.consumers;

            stream.consume(ClonableConsumer { consumers: consumers });
        }
    }
}

struct ClonableStream<S>
    where S: Stream
{
    consumers: Consumers<S::Item>,
    stream: S,
}

impl<S> ClonableStream<S>
    where S: Stream
{
    fn new(stream: S) -> Self {
        ClonableStream {
            consumers: Rc::new(RefCell::new(Vec::new())),
            stream: stream,
        }
    }
}

struct ClonableConsumer<T> {
    consumers: Consumers<T>,
}

impl<T> Consumer<T> for ClonableConsumer<T>
    where T: Clone
{
    fn emit(&mut self, item: T) -> bool {
        let mut consumers = self.consumers.borrow_mut();
        let mut i = consumers.len();

        while i > 0 {
            i -= 1;

            if !consumers[i].emit(item.clone()) {
                consumers.remove(i);
            }
        }

        consumers.len() > 0
    }
}
