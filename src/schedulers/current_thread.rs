use std::cell::RefCell;
use std::thread::sleep;
use std::time::{Duration, Instant};
use super::scheduler::*;

/// Schedule a func to be executed on the current thread.
#[derive(Copy, Clone)]
pub struct CurrentThread;

impl CurrentThread {
    pub fn current() -> CurrentThread {
        CurrentThread
    }

    pub fn is_running(&self) -> bool {
        CURRENT_THREAD.with(|f| f.borrow().is_running())
    }
}

struct Container {
    is_running: bool,
    is_sorted: bool,
    records: Vec<Box<Record>>,
}

fn pop_record(refcell: &RefCell<Container>) -> Option<Box<Record>> {
    refcell.borrow_mut().pop()
}

impl Container {
    fn is_running(&self) -> bool {
        self.is_running
    }

    fn new() -> Self {
        Container {
            is_running: false,
            is_sorted: true,
            records: Vec::new(),
        }
    }

    fn pop(&mut self) -> Option<Box<Record>> {
        if !self.is_sorted {
            self.records.sort_by_key(|r| r.get_instant());
            self.is_sorted = true;
        }

        let record = self.records.pop();
        self.is_running = record.is_some();

        record
    }

    fn schedule<F>(&mut self, func: F, delay: Duration) -> bool
        where F: FnOnce() + 'static
    {
        let due = Instant::now() + delay;

        self.records.push(Box::new(RecordItem::new(func, due)));
        self.is_sorted = false;

        self.is_running
    }
}

impl Scheduler for CurrentThread {
    fn schedule<F>(&mut self, func: F, delay: Duration)
        where F: FnOnce() + 'static
    {
        CURRENT_THREAD.with(|f| {
            let is_running = f.borrow_mut().schedule(func, delay);

            if !is_running {

                while let Some(mut record) = pop_record(f) {
                    let now = Instant::now();
                    let due = record.get_instant();

                    if due > now {
                        sleep(due - now);
                    }

                    record.invoke();
                }

            }
        });
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


thread_local!(static CURRENT_THREAD: RefCell<Container> = RefCell::new(Container::new()));
