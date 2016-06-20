use consumer::*;
use filter::*;
use inspect::*;
use min_by_key::*;
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

    /// Do something with each element of a stream, passing the value on.
    /// This is usefull to debug an item.
    fn inspect<F>(self, func: F) -> ParallelInspect<Self, F>
        where F: Send + Sync + Fn(&mut Self::Item),
              Self: Sized
    {
        ParallelInspect::new(self, func)
    }

    /// Returns the element that gives the minimum value from the specified
    /// function. Returns the lastest element if the comparison determines two
    /// elements to be equally minimum.
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    /// use asyncplify::schedulers::*;
    ///
    /// (0..10)
    ///     .into_stream()
    ///     .observe_on_parallel(EventLoop::new())
    ///     .min_by_key(|v| 10 - *v)
    ///     .subscribe_action(|v| assert!(v == 9, "v = {}", v));
    /// ```
    fn min_by_key<F, K>(self, f: F) -> ParallelMinByKey<Self, F>
        where Self: Sized,
              F: Send + Sync + Fn(&Self::Item) -> K,
              K: PartialOrd + Send
    {
        ParallelMinByKey::new(self, f)
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
