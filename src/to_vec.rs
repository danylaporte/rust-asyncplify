use consumer::*;
use std::mem::replace;
use stream::*;

struct ToVecState<C, F, T>
    where C: Consumer<Vec<T>>
{
    consumer: C,
    splitter: F,
    vec: Vec<T>,
}

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct ToVec<S, F> {
    stream: S,
    splitter: F,
}

impl<C, F, T> Drop for ToVecState<C, F, T>
    where C: Consumer<Vec<T>>
{
    fn drop(&mut self) {
        if !self.vec.is_empty() {
            let vec = replace(&mut self.vec, Vec::new());
            self.consumer.emit(vec);
        }
    }
}

impl<C, F, T> Consumer<T> for ToVecState<C, F, T>
    where C: Consumer<Vec<T>>,
          F: FnMut(&Vec<T>) -> bool
{
    fn emit(&mut self, item: T) -> bool {
        self.vec.push(item);

        if (self.splitter)(&self.vec) {
            let vec = replace(&mut self.vec, Vec::new());
            self.consumer.emit(vec)
        } else {
            true
        }
    }
}

impl<S, F, T> Stream<Vec<T>> for ToVec<S, F>
    where S: Stream<T>,
          F: FnMut(&Vec<T>) -> bool
{
    fn consume<C: Consumer<Vec<T>>>(self, consumer: C) {
        self.stream.consume(ToVecState {
            consumer: consumer,
            splitter: self.splitter,
            vec: Vec::new(),
        });
    }
}

pub trait ToVecStream<T>: Stream<T> {
    /// Bundle incoming elements into a `Vec`. A split function can be specified
    /// to emit a `Vec` when the splitter returns true. The remaing `Vec` is emited
    /// only when not empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    ///
    /// let mut v = Vec::new();
    ///
    /// (0..3)
    ///     .to_stream()
    ///     .to_vec(|vec| vec.len() == 2)  // split after 2 items
    ///     .inspect(|vec| v.push(vec.len()))
    ///     .subscribe();
    ///
    /// assert!(v == [2, 1], "v = {:?}", v);
    /// ```
    fn to_vec<F>(self, splitter: F) -> ToVec<Self, F>
        where F: FnMut(&Vec<T>) -> bool,
              Self: Sized
    {
        ToVec {
            stream: self,
            splitter: splitter,
        }
    }
}

impl<S, T> ToVecStream<T> for S where S: Stream<T> {}
