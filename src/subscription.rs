use consumer::*;

/// Represents a subscription to a `Stream`
pub struct Subscription {
    closed: bool,
}

impl Subscription {
    /// Closes the `Subscription`
    pub fn close(mut self) {
        self.closed = true;
    }

    pub fn new() -> Self {
        Subscription { closed: false }
    }
}

impl<T> Consumer<T> for Subscription {
    fn emit(&mut self, _: T) -> bool {
        !self.closed
    }
}