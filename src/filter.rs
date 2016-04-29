use consumer::*;
use producer::*;
use std::marker::PhantomData;
use std::rc::Rc;
use stream::*;

struct FilterState<C, F, T> {
    consumer: C,
    func: F,
    marker_t: PhantomData<T>,
}

impl<C, F, T> Consumer<T> for FilterState<C, F, T>
    where C: Consumer<T>,
          F: FnMut(&mut T) -> bool
{
    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, mut item: T) {
        if (self.func)(&mut item) {
            self.consumer.emit(item);
        }
    }
}

/// Describe a filter for a `stream`.
pub struct Filter<S, F, T> {
    stream: S,
    func: F,
    marker: PhantomData<T>,
}

impl<S, F, T> Stream<T> for Filter<S, F, T>
    where S: Stream<T>,
          F: FnMut(&mut T) -> bool
{
    fn consume<C: Consumer<T>>(self, consumer: C) {
        self.stream.consume(FilterState {
            consumer: consumer,
            func: self.func,
            marker_t: PhantomData::<T>,
        });
    }
}

/// Represent a filtrable `stream`.
pub trait FilterableStream<T>: Stream<T> {
    /// Filter a `Stream` based on a func `Stream`.
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    ///
    /// let mut sum = 0;
    ///
    /// (0..5)
    ///     .to_stream()
    ///     .filter(|v| *v > 2)
    ///     .tap(|v| sum += *v)
    ///     .subscribe();
    ///
    /// assert!(sum == 7, "sum (3 + 4) = {}", sum);
    /// ``` 
    fn filter<F>(self, func: F) -> Filter<Self, F, T>
        where Self: Sized,
              F: FnMut(&mut T) -> bool
    {
        Filter {
            stream: self,
            func: func,
            marker: PhantomData::<T>,
        }
    }
}

impl<S, T> FilterableStream<T> for S where S: Stream<T> {}
