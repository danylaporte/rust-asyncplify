use std::cell::RefCell;
use std::time::Duration;
use super::scheduler::*;
use super::event_loop::*;

/// Schedule a func to be executed on the current thread.
#[derive(Copy, Clone)]
pub struct CurrentThread;

impl CurrentThread {
    pub fn current() -> CurrentThread {
        CurrentThread
    }

    pub fn is_running(&self) -> bool {
        CURRENT_THREAD.with(|f| f.running())
    }
}

impl Scheduler for CurrentThread {
    fn schedule<F>(&self, func: F, delay: Duration)
        where F: FnOnce() + 'static
    {
        CURRENT_THREAD.with(|f| f.schedule(func, delay))
    }
}

thread_local!(static CURRENT_THREAD: RefCell<EventLoopContainer>
    = RefCell::new(EventLoopContainer::new()));
