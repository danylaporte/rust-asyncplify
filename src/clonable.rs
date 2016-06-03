use consumer::*;
use std::cell::RefCell;
use std::mem::replace;
use std::rc::Rc;
use stream::*;

type Consumers<T> = Rc<RefCell<Vec<Box<Consumer<T>>>>>;

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct Clonable<S, T>
    where S: Stream<T>,
          T: Clone
{
    stream: Option<Rc<ClonableStream<S, T>>>,
}

impl<S, T> Clonable<S, T>
    where S: Stream<T>,
          T: Clone
{
    pub fn new(stream: S) -> Self {
        Clonable { stream: Some(Rc::new(ClonableStream::new(stream))) }
    }
}

impl<S, T> Stream<T> for Clonable<S, T>
    where S: Stream<T>,
          T: Clone
{
    fn consume<C>(self, consumer: C)
        where C: Consumer<T> + 'static
    {
        if let Some(ref stream) = self.stream {
            stream.consumers.borrow_mut().push(Box::new(consumer));
        }
    }
}

impl<S, T> Clone for Clonable<S, T>
    where S: Stream<T>,
          T: Clone
{
    fn clone(&self) -> Self {
        Clonable { stream: self.stream.clone() }
    }
}

impl<S, T> Drop for Clonable<S, T>
    where S: Stream<T>,
          T: Clone
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

struct ClonableStream<S, T> {
    consumers: Consumers<T>,
    stream: S,
}

impl<S, T> ClonableStream<S, T> {
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
