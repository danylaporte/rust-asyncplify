use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant};
use super::scheduler::*;

#[derive(Clone)]
pub struct EventLoopScheduler {
    container: Arc<Mutex<EventLoopContainer>>,
}

impl EventLoopScheduler {
    /// Creates a new EventLoopScheduler
    pub fn new() -> Self {
        EventLoopScheduler { container: Arc::new(Mutex::new(EventLoopContainer::new())) }
    }
}

impl Scheduler for EventLoopScheduler {
    fn schedule<F>(&self, func: F, delay: Duration)
        where F: FnOnce() + 'static
    {
        self.container.schedule(func, delay)
    }
}

pub struct EventLoopContainer {
    records: Vec<Box<Record>>,
    running: bool,
    sorted: bool,
}

impl EventLoopContainer {
    pub fn new() -> Self {
        EventLoopContainer {
            records: Vec::new(),
            running: false,
            sorted: true,
        }
    }

    fn push(&mut self, record: Box<Record>) -> bool {
        self.records.push(record);
        self.sorted = false;

        self.running
    }

    fn pop(&mut self) -> Option<Box<Record>> {
        if !self.sorted {
            self.records.sort_by_key(|r| r.get_instant());
            self.sorted = true;
        }

        let record = self.records.pop();
        self.running = record.is_some();

        record
    }

    pub fn running(&self) -> bool {
        self.running
    }
}

pub trait EventLoop {
    fn push(&self, record: Box<Record>) -> bool;
    fn pop(&self) -> Option<Box<Record>>;
    fn running(&self) -> bool;

    fn schedule<F>(&self, func: F, delay: Duration)
        where F: FnOnce() + 'static
    {
        let due = Instant::now() + delay;
        let running = self.push(Box::new(RecordItem::new(func, due)));

        if !running {
            while let Some(mut record) = self.pop() {
                let now = Instant::now();
                let due = record.get_instant();

                if due > now {
                    sleep(due - now);
                }

                record.invoke();
            }
        }
    }
}

impl EventLoop for RefCell<EventLoopContainer> {
    fn push(&self, record: Box<Record>) -> bool {
        self.borrow_mut().push(record)
    }

    fn pop(&self) -> Option<Box<Record>> {
        self.borrow_mut().pop()
    }

    fn running(&self) -> bool {
        self.borrow().running()
    }
}

impl EventLoop for Mutex<EventLoopContainer> {
    fn push(&self, record: Box<Record>) -> bool {
        self.lock().unwrap().push(record)
    }

    fn pop(&self) -> Option<Box<Record>> {
        self.lock().unwrap().pop()
    }

    fn running(&self) -> bool {
        self.lock().unwrap().running()
    }
}

pub trait Record {
    fn get_instant(&self) -> Instant;
    fn invoke(&mut self);
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
