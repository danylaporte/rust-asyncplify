use std::cell::RefCell;
use std::time::{Duration, Instant};
use super::scheduler::*;
use std::thread::sleep;

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
    fn schedule(&mut self, item: Action, delay: Duration) {
        CURRENT_THREAD.with(|f| f.borrow_mut().schedule(item, delay));
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

    fn schedule_item(&mut self, action: Action, delay: Duration) {
        let due = Instant::now() + delay;
        self.records.push((due, action));
        self.set_due(due);
    }

    fn run_pending_actions(&mut self) {
        while !self.records.is_empty() {

            if let Some(due) = self.due {
                let sleep_delay = due - Instant::now();

                if sleep_delay > Duration::from_millis(0) {
                    sleep(sleep_delay);
                }
            }

            self.due = None;

            let mut i = self.records.len();
            self.records.sort_by_key(|v| v.0);

            while i > 0 {
                i -= 1;

                let due = self.records[i].0;

                if due <= Instant::now() {
                    let mut action = self.records.remove(i).1;

                    if let Some(delay) = action() {
                        self.schedule_item(action, delay);
                    }
                } else {
                    self.set_due(due);
                    break;
                }
            }
        }
    }

    fn schedule(&mut self, func: Action, delay: Duration) {
        self.schedule_item(func, delay);
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
