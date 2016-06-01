use std::time::Duration;

pub type Action = Box<FnMut() + Send>;

pub trait Scheduler {
    fn schedule(&mut self, func: Action, delay: Duration);
}