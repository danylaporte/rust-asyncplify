use std::cell::RefCell;
use std::thread::sleep;
use std::time::{Duration, Instant};
use super::schedule_queue::*;
use super::scheduler::*;

/// Schedule a func to be executed on the current thread.
#[derive(Copy, Clone)]
pub struct CurrentThread;

impl CurrentThread {
    pub fn current() -> CurrentThread {
        CurrentThread
    }

    pub fn running(&self) -> bool {
        CURRENT_THREAD.with(|f| f.borrow().running())
    }
}

struct Container {
    queue: ScheduleQueue<Box<Action>>,
    running: bool,
}

impl Container {
    fn new() -> Self {
        Container {
            queue: ScheduleQueue::new(),
            running: false,
        }
    }

    fn dequeue(&mut self) -> Option<(Box<Action>, Instant)> {
        if let Some(record) = self.queue.dequeue() {
            self.running = true;
            Some(record)
        } else {
            self.running = false;
            None
        }
    }

    fn enqueue(&mut self, record: (Box<Action>, Instant)) -> bool {
        self.queue.enqueue(record);
        self.running
    }

    fn running(&self) -> bool {
        self.running
    }
}

impl Scheduler for CurrentThread {
    fn schedule<F>(&self, func: F, delay: Duration)
        where F: FnOnce() + 'static
    {
        let due = Instant::now() + delay;
        let running = CURRENT_THREAD.with(|f| f.borrow_mut().enqueue((Box::new(Some(func)), due)));

        if !running {
            while let Some(mut record) = CURRENT_THREAD.with(|f| f.borrow_mut().dequeue()) {
                let now = Instant::now();
                let due = record.1;

                if due > now {
                    sleep(due - now);
                }

                record.0.invoke();
            }
        }
    }
}

trait Action {
    fn invoke(&mut self);
}

impl<F> Action for Option<F>
    where F: FnOnce()
{
    fn invoke(&mut self) {
        if let Some(func) = self.take() {
            func();
        }
    }
}

thread_local!(static CURRENT_THREAD: RefCell<Container>
    = RefCell::new(Container::new()));
