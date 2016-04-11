use consumer::*;
use producer::*;
use std::marker::PhantomData;
use std::rc::Rc;
use stream::*;

struct FilterState<'a, F, T: 'a> {
    consumer: &'a mut Consumer<T>,
    func: F,
}

impl<'a, F, T> Consumer<T> for FilterState<'a, F, T>
    where F: FnMut(&T) -> bool
{
    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: T) {
        if (self.func)(&item) {
            self.consumer.emit(item);
        }
    }

    fn end(&mut self) {
        self.consumer.end();
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
          F: FnMut(&T) -> bool
{
    fn consume(self, consumer: &mut Consumer<T>) {
        self.stream.consume(&mut FilterState {
            consumer: consumer,
            func: self.func,
        });
    }
}

/// Represent a filtrable `stream`.
pub trait FilterableStream<T> : Stream<T> {
    /// Filter a `Stream` based on a func `Stream`.
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;    
    ///
    /// (0..10)
    ///     .to_stream()
    ///     .filter(|v| v % 2 == 0)
    ///     .subscribe();
    /// ``` 
    fn filter<F>(self, func: F) -> Filter<Self, F, T>
        where Self: Sized,
              F: FnMut(&T) -> bool
    {
        Filter {
            stream: self,
            func: func,
            marker: PhantomData::<T>,
        }
    }
}

impl<S, T> FilterableStream<T> for S where S: Stream<T> {}

#[cfg(test)]
mod tests {
    use fold::*;
    use iter::*;
    use subscription::*;
    use super::*;
    use tap::*;

    #[test]
    fn it_works() {
        let mut f = 0;
        (0..10)
            .to_stream()
            .filter(|v| *v < 4)
            .sum()
            .tap(|v| f = *v)
            .subscribe();

        assert!(f == 6, "f = {}", f);
    }
}
