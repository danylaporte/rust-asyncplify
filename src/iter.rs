use consumer::*;
use stream::*;

/// Represent a stream on an iterator.
#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct IterStream<I> {
    iterator: I,
}

impl<I, T> Stream<T> for IterStream<I>
    where I: Iterator<Item = T>
{
    fn consume<C: Consumer<T>>(self, mut consumer: C) {
        for i in self.iterator {
            if !consumer.emit(i) {
                break;
            }
        }
    }
}

/// Extend the Iterator trait with stream conversion operators
pub trait ToStream: Iterator {
    /// Convert an iterator to a `Stream`.
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    ///
    /// let vec = (0..5)
    ///     .to_stream()
    ///     .into_vec();
    ///
    /// assert!(vec == [0, 1, 2, 3, 4], "vec = {:?}", vec);
    /// ``` 
    fn to_stream(self) -> IterStream<Self>
        where Self: Sized
    {
        IterStream { iterator: self }
    }
}

impl<I> ToStream for I where I: Iterator {}
