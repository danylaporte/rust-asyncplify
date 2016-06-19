use consumer::*;
use filter::*;
use observe_on::*;
use subscription::*;
use super::schedulers::ParallelScheduler;

pub trait ParallelStream {
    type Item: Send;

    fn consume<C>(self, C) where C: ParallelConsumer<Self::Item>;

    /// Creates a stream which uses a closure to determine if an element should
    /// be emitted. The closure must return true or false. `filter()` creates a
    /// stream which calls this closure on each element. If the closure returns
    /// true, then the element is returned. If the closure returns false, it
    /// will try again, and call the closure on the next element, seeing if it
    /// passes the test.
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    /// use asyncplify::schedulers::*;
    ///
    /// (0..4)
    ///     .into_stream()
    ///     .observe_on_parallel(EventLoop::new())
    ///     .filter(|v| *v > 2)
    ///     .subscribe_action(|v| assert!(v == 3, "v = {:?}", v))
    ///
    /// ```
    fn filter<F>(self, predicate: F) -> ParallelFilter<Self, F>
        where Self: Sized,
              F: Send + Fn(&mut Self::Item) -> bool
    {
        ParallelFilter::new(self, predicate)
    }


    fn observe_on<SC>(self, scheduler: SC) -> ParallelObserveOn<Self, SC>
        where SC: ParallelScheduler,
              Self: Sized
    {
        ParallelObserveOn::new(self, scheduler)
    }

    fn subscribe(self)
        where Self: Sized
    {
        self.consume(Subscription);
    }

    fn subscribe_action<F>(self, action: F)
        where Self: Sized,
              F: Fn(Self::Item) + Send + Sync
    {
        self.consume(SubscriptionAction::new(action));
    }

    fn subscribe_func<F>(self, predicate: F)
        where Self: Sized,
              F: Send + Sync + Fn(Self::Item) -> bool
    {
        self.consume(SubscriptionFunc::new(predicate));
    }
}
