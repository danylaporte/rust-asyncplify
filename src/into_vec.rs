use consumer::*;
use producer::*;
use std::rc::Rc;
use stream::*;

impl<'a, T> Consumer<T> for &'a mut Vec<T> {
    fn init(&mut self, _: Rc<Producer>) {}
    fn emit(&mut self, item: T) {
        self.push(item);
    }
}

pub trait IntoVecStream<T>: Stream<T> {
    /// Convert the stream into a `Vec`
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    ///
    /// let vec = (0..5).to_stream().into_vec();
    /// assert!(vec == [0, 1, 2, 3, 4], "vec = {:?}", vec);
    /// ```
    fn into_vec(self) -> Vec<T>
        where Self: Sized
    {
        let mut v = Vec::new();
        self.consume(&mut v);
        v
    }
}

impl<S, T> IntoVecStream<T> for S where S: Stream<T> {}