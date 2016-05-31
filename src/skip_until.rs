use consumer::*;
use std::cell::Cell;
use std::rc::Rc;
use stream::*;

/// Ignores items until another stream emit one item.
///
/// This struct is created by the [skip_until()](./trait.Stream.html#method.skip_until) method on [Stream](./trait.Stream.html).
/// See its documentation for more.
#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct SkipUntil<S, T> {
    stream: S,
    trigger: T,
}

struct SkipUntilState<C> {
    consumer: C,
    is_opened: Rc<Cell<bool>>,
}

struct TriggerConsumer {
    is_opened: Rc<Cell<bool>>,
}

impl<T> Consumer<T> for TriggerConsumer {
    fn emit(&mut self, _: T) -> bool {
        self.is_opened.set(true);
        false
    }
}

impl<C, T> Consumer<T> for SkipUntilState<C>
    where C: Consumer<T>
{
    fn emit(&mut self, item: T) -> bool {
        !self.is_opened.get() || self.consumer.emit(item)
    }
}

impl<S, T, U> Stream<T> for SkipUntil<S, U>
    where S: Stream<T>,
          U: Stream<()>
{
    fn consume<C>(self, consumer: C)
        where C: Consumer<T>
    {
        let is_opened = Rc::new(Cell::new(false));

        self.trigger.consume(TriggerConsumer { is_opened: is_opened.clone() });

        self.stream.consume(SkipUntilState {
            consumer: consumer,
            is_opened: is_opened,
        });
    }
}

impl<S, T> SkipUntil<S, T> {
    pub fn new(stream: S, trigger: T) -> Self {
        SkipUntil {
            stream: stream,
            trigger: trigger,
        }
    }
}