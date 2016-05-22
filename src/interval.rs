use consumer::*;
use stream::*;
use std::thread;
use std::time::Duration;

pub struct Interval {
    period: Duration,
}

impl Interval {
    /// Constructs a new `Stream` that emit an empty tuple after each period.
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    /// use std::time::*;
    ///
    /// let mut count = 0;
    /// let start = Instant::now();
    ///
    /// Interval::new(Duration::from_millis(5))
    ///     .take(2)
    ///     .inspect(|_| count += 1)
    ///     .subscribe();
    ///
    /// assert!(count == 2, "count = {}", count);
    /// let d = Instant::now().duration_since(start);
    /// assert!(d > Duration::from_millis(10), "d == {:?}", d);
    /// ```
    pub fn new(period: Duration) -> Self {
        Interval { period: period }
    }
}

impl Stream<()> for Interval {
    fn consume<C>(self, mut consumer: C)
        where C: Consumer<()>
    {
        loop {
            thread::sleep(self.period);

            if !consumer.emit(()) {
                break;
            }
        }
    }
}