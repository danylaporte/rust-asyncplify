use consumer::*;
use stream::*;

/// Constructs a new empty `Stream` based on a `Empty`.
///
/// # Examples
///
/// ```
/// use asyncplify::*;
///
/// let mut count = 0;
///
/// Empty
///     .inspect(|_| count += 1)
///     .subscribe();
///
/// assert!(count == 0, "count = {}", count);
/// ```
#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct Empty;

impl Stream<()> for Empty {
    fn consume<C: Consumer<()>>(self, _: C) {}
}
