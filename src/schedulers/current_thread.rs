use std::cell::RefCell;
use std::time::{Duration, Instant};
use super::scheduler::*;

pub struct CurrentThread;

impl CurrentThread {
    pub fn current() -> CurrentThread {
        CurrentThread
    }

    pub fn has_pending_actions(&self) -> bool {
        CURRENT_THREAD.with(|f| f.borrow().has_pending_actions())
    }

    pub fn run_pending_actions(&mut self) {
        CURRENT_THREAD.with(|f| f.borrow_mut().run_pending_actions())
    }
}

impl Scheduler for CurrentThread {
    fn schedule(&mut self, func: Action, delay: Duration) {
        CURRENT_THREAD.with(|f| f.borrow_mut().schedule(func, delay));
    }
}

struct ThreadCell {
    due: Option<Instant>,
    records: Vec<(Instant, Action)>,
}

impl ThreadCell {
    fn has_pending_actions(&self) -> bool {
        if let Some(due) = self.due {
            due <= Instant::now()
        } else {
            false
        }
    }

    fn new() -> Self {
        ThreadCell {
            due: None,
            records: Vec::new(),
        }
    }

    fn run_pending_actions(&mut self) {

        if self.has_pending_actions() {
            self.records.sort_by_key(|v| v.0);

            let mut i = self.records.len();
            let instant = Instant::now();
            self.due = None;

            while i > 0 {
                i -= 1;

                let due = self.records[i].0;

                if due <= instant {
                    let mut action = self.records.remove(i).1;
                    action();
                } else {
                    self.set_due(due);
                    break;
                }
            }
        }
    }

    fn schedule(&mut self, func: Action, delay: Duration) {
        let mut instant = Instant::now();
        instant += delay;

        self.records.push((instant, func));
        self.set_due(instant);
        self.run_pending_actions();
    }

    fn set_due(&mut self, instant: Instant) {
        if let Some(due) = self.due {
            if due > instant {
                self.due = Some(instant);
            }
        }
    }
}

thread_local!(static CURRENT_THREAD: RefCell<ThreadCell> = RefCell::new(ThreadCell::new()));