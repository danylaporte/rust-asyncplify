use consumer::*;
use stream::*;

/// Represent a stream on an iterator.
#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct IterStream<I> {
    iterator: I,
}

impl<I> Stream for IterStream<I>
    where I: Iterator
{
    type Item = I::Item;

    fn consume<C>(self, mut consumer: C)
        where C: Consumer<Self::Item>
    {
        for i in self.iterator {
            if !consumer.emit(i) {
                break;
            }
        }
    }
}

/// Extend the Iterator trait with stream conversion operators
pub trait IntoStream: Iterator {
    /// Convert an iterator to a `Stream`.
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    ///
    /// let vec = (0..5)
    ///     .into_stream()
    ///     .into_vec();
    ///
    /// assert!(vec == [0, 1, 2, 3, 4], "vec = {:?}", vec);
    /// ```
    fn into_stream(self) -> IterStream<Self>
        where Self: Sized
    {
        IterStream { iterator: self }
    }
}

impl<I> IntoStream for I where I: Iterator {}
