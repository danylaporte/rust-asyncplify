use consumer::*;
use parallel_stream::*;
use stream::*;

struct FilterState<C, F> {
    consumer: C,
    predicate: F,
}

impl<C, F, T> Consumer<T> for FilterState<C, F>
    where C: Consumer<T>,
          F: FnMut(&mut T) -> bool
{
    fn emit(&mut self, mut item: T) -> bool {
        if (self.predicate)(&mut item) {
            self.consumer.emit(item)
        } else {
            true
        }
    }
}

impl<C, F, T> ParallelConsumer<T> for FilterState<C, F>
    where C: ParallelConsumer<T>,
          F: Send + Sync + Fn(&mut T) -> bool,
          T: Send
{
    fn emit(&self, mut item: T) -> bool {
        if (self.predicate)(&mut item) {
            self.consumer.emit(item)
        } else {
            true
        }
    }
}

/// This struct is created by the
/// [`filter()`](./trait.Stream.html#method.filter) method on
/// [Stream](./trait.Stream.html). See its documentation for more.
#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct Filter<S, F> {
    stream: S,
    predicate: F,
}

/// This struct is created by the
/// [`filter()`](./trait.ParallelStream.html#method.filter) method on
/// [ParallelStream](./trait.ParallelStream.html). See its documentation for more.
#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct ParallelFilter<S, F> {
    stream: S,
    predicate: F,
}

impl<S, F> Stream for Filter<S, F>
    where S: Stream,
          F: FnMut(&mut S::Item) -> bool
{
    type Item = S::Item;

    fn consume<C>(self, consumer: C)
        where C: Consumer<Self::Item>
    {
        self.stream.consume(FilterState {
            consumer: consumer,
            predicate: self.predicate,
        });
    }
}

impl<S, F> ParallelStream for ParallelFilter<S, F>
    where S: ParallelStream,
          F: Send + Sync + Fn(&mut S::Item) -> bool,
          S::Item: Send
{
    type Item = S::Item;

    fn consume<C>(self, consumer: C)
        where C: ParallelConsumer<Self::Item>
    {
        self.stream.consume(FilterState {
            consumer: consumer,
            predicate: self.predicate,
        });
    }
}

impl<S, F> Filter<S, F> {
    pub fn new(stream: S, predicate: F) -> Self {
        Filter {
            predicate: predicate,
            stream: stream,
        }
    }
}

impl<S, F> ParallelFilter<S, F> {
    pub fn new(stream: S, predicate: F) -> Self {
        ParallelFilter {
            predicate: predicate,
            stream: stream,
        }
    }
}
