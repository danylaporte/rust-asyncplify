use consumer::*;
use stream::*;

/// A stream that emits an element exactly once.
///
/// This `struct` is created by the [`once()`] function. See its documentation
/// for more.
///
/// [`once()`]: fn.once.html
#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct Once<T> {
    value: T,
}

/// Creates a stream that emits an element exactly once.
///
/// # Examples
///
/// ```
/// use asyncplify::*;
///
/// let vec = once(5).into_vec();
/// assert!(vec == [5], "vec == {:?}", vec);
/// ```
pub fn once<T>(value: T) -> Once<T> {
    Once { value: value }
}

impl<T> Stream for Once<T> {
    type Item = T;

    fn consume<C>(self, mut consumer: C)
        where C: Consumer<Self::Item>
    {
        consumer.emit(self.value);
    }
}
