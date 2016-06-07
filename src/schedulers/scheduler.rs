use std::time::Duration;

pub trait Scheduler {
    fn schedule<F>(&mut self, func: F, delay: Duration) where F: FnOnce() + 'static;
}
