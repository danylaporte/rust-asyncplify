use consumer::*;
use std::cmp::Ord;
use std::collections::*;
use stream::*;

impl<'a, T> Consumer<T> for &'a mut BinaryHeap<T>
    where T: Ord
{
    fn emit(&mut self, item: T) -> bool {
        self.push(item);
        true
    }
}

impl<'a, T> Consumer<T> for &'a mut LinkedList<T> {
    fn emit(&mut self, item: T) -> bool {
        self.push_front(item);
        true
    }
}

impl<'a, T> Consumer<T> for &'a mut Vec<T> {
    fn emit(&mut self, item: T) -> bool {
        self.push(item);
        true
    }
}

impl<'a, T> Consumer<T> for &'a mut VecDeque<T> {
    fn emit(&mut self, item: T) -> bool {
        self.push_front(item);
        true
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
    /// let vec = (0..5).into_stream().into_vec();
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