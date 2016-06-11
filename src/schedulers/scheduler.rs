use std::time::Duration;

pub trait Scheduler {
    fn schedule<F>(&self, func: F, delay: Duration) where F: FnOnce() + 'static;
}
