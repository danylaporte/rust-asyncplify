use std::cell::RefCell;
use std::thread::sleep;
use std::time::{Duration, Instant};
use super::scheduler::*;

/// Schedule a func to be executed on the current thread. The action is not
/// executed immediately but is queued and execute when calling one of the
/// methods [`run_pending_actions()`](#method.run_pending_actions),
/// [`run_until_empty()`](#method.run_until_empty).
#[derive(Copy, Clone)]
pub struct CurrentThread;

impl CurrentThread {
    pub fn current() -> CurrentThread {
        CurrentThread
    }

    /// Returns true if the schedule queue contains funcs that are due
    /// to be executed.
    pub fn has_pending_actions(&self) -> bool {
        CURRENT_THREAD.with(|f| f.borrow().has_pending_actions())
    }

    /// Returns true if the schedule queue is empty
    pub fn is_empty(&self) -> bool {
        CURRENT_THREAD.with(|f| f.borrow().is_empty())
    }

    /// Block and execute only the actions that are due to be executed.
    pub fn run_pending_actions(&mut self) {
        CURRENT_THREAD.with(|f| f.borrow_mut().run_pending_actions())
    }

    /// Block and execute all items in the schedule queue until it is completed.
    pub fn run_until_empty(&mut self) {
        CURRENT_THREAD.with(|f| f.borrow_mut().run_until_empty())
    }
}

impl Scheduler for CurrentThread {
    fn schedule<F>(&mut self, func: F, delay: Duration)
        where F: FnOnce() + 'static
    {
        CURRENT_THREAD.with(|f| f.borrow_mut().schedule(func, delay))
    }
}

struct RecordItem<F> {
    func: Option<F>,
    instant: Instant,
}

impl<F> RecordItem<F>
    where F: FnOnce() + 'static
{
    fn new(f: F, instant: Instant) -> Self {
        RecordItem {
            func: Some(f),
            instant: instant,
        }
    }
}

trait Record {
    fn get_instant(&self) -> Instant;
    fn invoke(&mut self);
}

impl<F> Record for RecordItem<F>
    where F: FnOnce()
{
    fn get_instant(&self) -> Instant {
        self.instant
    }

    fn invoke(&mut self) {
        if let Some(func) = self.func.take() {
            func();
        }
    }
}

struct ThreadCell {
    due: Option<Instant>,
    records: Vec<Box<Record>>,
}

impl ThreadCell {
    fn has_pending_actions(&self) -> bool {
        if let Some(due) = self.due {
            due <= Instant::now()
        } else {
            false
        }
    }

    fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    fn new() -> Self {
        ThreadCell {
            due: None,
            records: Vec::new(),
        }
    }

    fn run_once(&mut self) {
        self.records.sort_by_key(|r| r.get_instant());
        self.due = None;

        let mut i = self.records.len();

        while i > 0 {
            i -= 1;

            let due = self.records[i].get_instant();

            if due <= Instant::now() {
                self.records.remove(i).invoke();

            } else {
                self.set_due(due);
                break;
            }
        }
    }

    fn run_pending_actions(&mut self) {
        while self.due.map_or(false, |v| v <= Instant::now()) {
            self.run_once();
        }
    }

    fn run_until_empty(&mut self) {
        while let Some(due) = self.due.take() {
            let now = Instant::now();
            if due > now {
                sleep(due - now);
            }
            self.run_once();
        }
    }

    fn schedule<F>(&mut self, func: F, delay: Duration)
        where F: FnOnce() + 'static
    {
        let due = Instant::now() + delay;
        self.records.push(Box::new(RecordItem::new(func, due)));
        self.set_due(due);
    }

    fn set_due(&mut self, instant: Instant) {
        if let Some(due) = self.due {
            if due > instant {
                self.due = Some(instant);
            }
        } else {
            self.due = Some(instant);
        }
    }
}

thread_local!(static CURRENT_THREAD: RefCell<ThreadCell> = RefCell::new(ThreadCell::new()));
