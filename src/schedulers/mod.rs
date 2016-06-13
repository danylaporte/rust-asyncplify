//! If you want to introduce multiple threads or delay, you can do so by using schedulers.
//!

mod current_thread;
mod event_loop;
mod schedule_queue;
mod scheduler;

pub use self::current_thread::*;
pub use self::event_loop::EventLoopScheduler;
pub use self::scheduler::*;
