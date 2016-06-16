use consumer::*;

/// This struct is created by the
/// [`subscribe()`](./trait.Stream.html#method.subscribe) method on
/// [Stream](./trait.Stream.html). See its documentation for more.
pub struct Subscription;

/// This struct is created by the
/// [`subscribe_action()`](./trait.Stream.html#method.subscribe_action) method on
/// [Stream](./trait.Stream.html). See its documentation for more.
pub struct SubscriptionAction<F> {
    f: F,
}

/// This struct is created by the
/// [`subscribe_func()`](./trait.Stream.html#method.subscribe_func) method on
/// [Stream](./trait.Stream.html). See its documentation for more.
pub struct SubscriptionFunc<F> {
    predicate: F,
}

impl<F> SubscriptionAction<F> {
    /// Creates a new `[SubscriptionAction](./struct.SubscriptionAction.html)`
    pub fn new(f: F) -> Self {
        SubscriptionAction { f: f }
    }
}

impl<F> SubscriptionFunc<F> {
    /// Creates a new `[SubscriptionFunc](./struct.SubscriptionFunc.html)`
    pub fn new(f: F) -> Self {
        SubscriptionFunc { predicate: f }
    }
}

impl<T> Consumer<T> for Subscription {
    fn emit(&mut self, _: T) -> bool {
        true
    }
}

impl<T> ParallelConsumer<T> for Subscription
    where T: Send
{
    fn emit(&self, _: T) -> bool {
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

impl<F, T> ParallelConsumer<T> for SubscriptionAction<F>
    where F: Fn(T) + Send + Sync,
          T: Send
{
    fn emit(&self, item: T) -> bool {
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

impl<F, T> ParallelConsumer<T> for SubscriptionFunc<F>
    where F: Send + Sync + Fn(T) -> bool,
          T: Send
{
    fn emit(&self, item: T) -> bool {
        (self.predicate)(item)
    }
}
