use atom::Atom;
use consumer::*;
use parallel_stream::*;
use std::cmp::PartialOrd;
use stream::*;
use utils::replace_until_change;

struct MaxState<C, T>
    where C: Consumer<T>,
          T: PartialOrd
{
    consumer: C,
    value: Option<T>,
}

impl<C, T> Consumer<T> for MaxState<C, T>
    where C: Consumer<T>,
          T: PartialOrd
{
    fn emit(&mut self, mut item: T) -> bool {

        if let Some(current) = self.value.take() {
            if current > item {
                item = current;
            }
        }

        self.value = Some(item);
        true
    }
}

impl<C, T> Drop for MaxState<C, T>
    where C: Consumer<T>,
          T: PartialOrd
{
    fn drop(&mut self) {
        if let Some(value) = self.value.take() {
            self.consumer.emit(value);
        }
    }
}

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct Max<S> {
    stream: S,
}

impl<S> Stream for Max<S>
    where S: Stream,
          S::Item: PartialOrd
{
    type Item = S::Item;

    fn consume<C>(self, consumer: C)
        where C: Consumer<Self::Item>
    {
        self.stream.consume(MaxState {
            consumer: consumer,
            value: None,
        });
    }
}

impl<S> Max<S> {
    pub fn new(stream: S) -> Self {
        Max { stream: stream }
    }
}

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct ParallelMax<S> {
    stream: S,
}

impl<S> ParallelStream for ParallelMax<S>
    where S: ParallelStream,
          S::Item: PartialOrd + Send
{
    type Item = S::Item;

    fn consume<C>(self, consumer: C)
        where C: ParallelConsumer<Self::Item>
    {
        self.stream.consume(ParallelMaxState {
            consumer: consumer,
            value: Atom::empty(),
        });
    }
}

impl<S> ParallelMax<S> {
    pub fn new(stream: S) -> Self {
        ParallelMax { stream: stream }
    }
}

struct ParallelMaxState<C, T>
    where C: ParallelConsumer<T>,
          T: PartialOrd + Send
{
    consumer: C,
    value: Atom<Box<T>>,
}

impl<C, T> ParallelConsumer<T> for ParallelMaxState<C, T>
    where C: ParallelConsumer<T>,
          T: PartialOrd + Send
{
    fn emit(&self, item: T) -> bool {
        replace_until_change(&self.value, item, |current, item| if *current > *item {
            current
        } else {
            item
        });
        true
    }
}

impl<C, T> Drop for ParallelMaxState<C, T>
    where C: ParallelConsumer<T>,
          T: PartialOrd + Send
{
    fn drop(&mut self) {
        if let Some(value) = self.value.take() {
            self.consumer.emit(*value);
        }
    }
}