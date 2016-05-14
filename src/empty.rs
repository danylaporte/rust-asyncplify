use consumer::*;
use producer::*;
use std::rc::Rc;
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
pub struct Empty;

impl Stream<()> for Empty {
    fn consume<C: Consumer<()>>(self, mut consumer: C) {
        consumer.init(Rc::new(Producer::new()));
    }
}