use std::sync::{Arc, Condvar, Mutex};
use std::thread::spawn;
use std::time::{Duration, Instant};
use super::schedule_queue::*;
use super::scheduler::*;

#[derive(Clone)]
pub struct EventLoop {
    queue: Arc<(Mutex<ScheduleQueue<Box<Action + Send>>>, Condvar)>,
}

impl EventLoop {
    /// Creates a new EventLoop
    pub fn new() -> Self {
        let queue = Arc::new((Mutex::new(ScheduleQueue::new()), Condvar::new()));

        let scheduler = EventLoop { queue: queue.clone() };

        spawn(move || {
            loop {
                let mut action = dequeue(&queue);
                action.invoke();
            }
        });

        scheduler
    }
}

fn dequeue(queue: &Arc<(Mutex<ScheduleQueue<Box<Action + Send>>>, Condvar)>) -> Box<Action> {
    let (ref mutex, ref cvar) = **queue;
    let mut queue = mutex.lock().unwrap();

    loop {
        if let Some(record) = queue.dequeue() {
            let now = Instant::now();

            if record.1 <= now {
                return record.0;
            } else {
                let timeout = now - record.1;
                let r = cvar.wait_timeout(queue, timeout).unwrap();
                queue = r.0;

                if r.1.timed_out() {
                    return record.0;
                } else {
                    queue.enqueue(record);
                    continue;
                }
            }
        } else {
            queue = cvar.wait(queue).unwrap();
        }
    }
}

impl ParallelScheduler for EventLoop {
    fn schedule<F>(&self, func: F, delay: Duration)
        where F: FnOnce() + Send + 'static
    {
        let due = Instant::now() + delay;
        let &(ref mutex, ref cvar) = &*self.queue;

        mutex.lock().unwrap().enqueue((Box::new(Some(func)), due));
        cvar.notify_one();
    }
}

trait Action {
    fn invoke(&mut self);
}

impl<F> Action for Option<F>
    where F: FnOnce() + Send
{
    fn invoke(&mut self) {
        if let Some(action) = self.take() {
            action();
        }
    }
}
