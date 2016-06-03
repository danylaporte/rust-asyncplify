use std::time::Duration;

pub type Action = Box<FnMut() -> Option<Duration>>;

pub trait Scheduler {
    fn schedule(&mut self, item: Action, delay: Duration);
}
