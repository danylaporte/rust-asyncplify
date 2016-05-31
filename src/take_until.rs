use consumer::*;
use std::cell::Cell;
use std::rc::Rc;
use stream::*;

/// A stream that only emit items until it receive an item from another stream.
///
/// This struct is created by the [take_until()](./trait.Stream.html#method.take_until) method on [Stream](./trait.Stream.html).
/// See its documentation for more.
#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct TakeUntil<S, T> {
    stream: S,
    trigger: T,
}

struct TakeUntilState<C> {
    consumer: C,
    is_closed: Rc<Cell<bool>>,
}

struct TriggerConsumer {
    is_closed: Rc<Cell<bool>>,
}

impl<T> Consumer<T> for TriggerConsumer {
    fn emit(&mut self, _: T) -> bool {
        self.is_closed.set(true);
        false
    }
}

impl<C, T> Consumer<T> for TakeUntilState<C>
    where C: Consumer<T>
{
    fn emit(&mut self, item: T) -> bool {
        !self.is_closed.get() && self.consumer.emit(item)
    }
}

impl<S, T, U> Stream<T> for TakeUntil<S, U>
    where S: Stream<T>,
          U: Stream<()>
{
    fn consume<C>(self, consumer: C)
        where C: Consumer<T>
    {
        let is_closed = Rc::new(Cell::new(false));

        self.trigger.consume(TriggerConsumer { is_closed: is_closed.clone() });

        self.stream.consume(TakeUntilState {
            consumer: consumer,
            is_closed: is_closed,
        });
    }
}

impl<S, T> TakeUntil<S, T> {
    pub fn new(stream: S, trigger: T) -> Self {
        TakeUntil {
            stream: stream,
            trigger: trigger,
        }
    }
}