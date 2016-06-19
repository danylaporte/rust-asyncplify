use consumer::*;
use parallel_stream::*;
use stream::*;

/// A [`Stream`](./trait.Stream.html) that do not emits any element.
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

impl Stream for Empty {
    type Item = ();

    fn consume<C: Consumer<()>>(self, _: C) {}
}

/// A [`ParallelStream`](./trait.ParallelStream.html) that do not emits any element.
///
/// # Examples
///
/// ```
/// use asyncplify::*;
/// use std::sync::atomic::{AtomicBool, Ordering};
/// use std::sync::Arc;
///
/// let found = Arc::new(AtomicBool::new(false));
///
/// ParallelEmpty
///     .inspect(|_| found.store(true, Ordering::Release))
///     .subscribe();
///
/// assert!(!found.load(Ordering::Acquire), "found = {}", found.load(Ordering::Acquire));
/// ```
#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct ParallelEmpty;

impl ParallelStream for ParallelEmpty {
    type Item = ();

    fn consume<C: ParallelConsumer<()>>(self, _: C) {}
}
