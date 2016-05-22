use consumer::*;

/// Represents a subscription to a `Stream`
pub struct Subscription;

/// Represents a subscription to a `Stream` based on an action
pub struct SubscriptionAction<F> {
    f: F,
}

/// Represents a subscription to a `Stream` based on a func
pub struct SubscriptionFunc<F> {
    predicate: F,
}

impl<F> SubscriptionAction<F> {
    /// Creates a new `SubscriptionAction`
    pub fn new(f: F) -> Self {
        SubscriptionAction { f: f }
    }
}

impl<F> SubscriptionFunc<F> {
    /// Creates a new `SubscriptionFunc`
    pub fn new(f: F) -> Self {
        SubscriptionFunc { predicate: f }
    }
}

impl<T> Consumer<T> for Subscription {
    fn emit(&mut self, _: T) -> bool {
        true
    }
}

impl<F, T> Consumer<T> for SubscriptionAction<F>
    where F: FnMut(T)
{
    fn emit(&mut self, item: T) -> bool {
        (self.f)(item);
        true
    }
}

impl<F, T> Consumer<T> for SubscriptionFunc<F>
    where F: FnMut(T) -> bool
{
    fn emit(&mut self, item: T) -> bool {
        (self.predicate)(item)
    }
}