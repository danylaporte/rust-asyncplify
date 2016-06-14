use consumer::*;
use observe_on::*;
use subscription::*;
use super::schedulers::ParallelScheduler;

pub trait ParallelStream<T: Send>: Send {
    fn consume<C: ParallelConsumer<T>>(self, C);

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
              F: Fn(T) + Send
    {
        self.consume(SubscriptionAction::new(action));
    }

    fn subscribe_func<F>(self, predicate: F)
        where Self: Sized,
              F: Send + Fn(T) -> bool
    {
        self.consume(SubscriptionFunc::new(predicate));
    }
}