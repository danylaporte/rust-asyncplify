use consumer::*;
use producer::*;
use std::collections::VecDeque;
use std::rc::Rc;
use stream::*;

struct TakeLastState<C, T>
    where C: Consumer<T>
{
    consumer: C,
    count: usize,
    producer: Option<Rc<Producer>>,
    queue: VecDeque<T>,
}

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct TakeLast<S> {
    count: usize,
    stream: S,
}

impl<C, T> Consumer<T> for TakeLastState<C, T>
    where C: Consumer<T>
{
    fn init(&mut self, producer: Rc<Producer>) {
        self.producer = Some(producer.clone());
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: T) {
        if self.count == self.queue.len() {
            self.queue.pop_front();
        }
        self.queue.push_back(item);
    }
}

impl<C, T> Drop for TakeLastState<C, T>
    where C: Consumer<T>
{
    fn drop(&mut self) {
        if let Some(ref producer) = self.producer {
            for i in self.queue.drain(..) {
                if !producer.is_closed() {
                    self.consumer.emit(i);
                }
            }
        }
    }
}

impl<S, T> Stream<T> for TakeLast<S>
    where S: Stream<T>
{
    fn consume<C: Consumer<T>>(self, consumer: C) {
        if self.count == 0 {
            self.stream.consume(consumer);
        } else {
            self.stream.consume(TakeLastState {
                consumer: consumer,
                count: self.count,
                producer: None,
                queue: VecDeque::new(),
            });
        }
    }
}

impl<S> TakeLast<S> {
    pub fn new(stream: S, count: usize) -> Self {
        TakeLast {
            count: count,
            stream: stream,
        }
    }
}
