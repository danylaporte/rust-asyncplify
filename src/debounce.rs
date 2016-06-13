use atom::*;
use consumer::*;
use schedulers::{CurrentThread, Scheduler};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use stream::*;

/// Only emit an item from a [Stream](./trait.Stream.html) if a particular
/// duration has passed without it emitting another item.
///
/// This struct is created by the [debounce()](./trait.Stream.html#method.debounce)
/// method on [Stream](./trait.Stream.html).
/// See its documentation for more.
#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct Debounce<S, SC> {
    delay: Duration,
    scheduler: SC,
    stream: S,
}

impl<S, SC> Debounce<S, SC> {
    pub fn new(stream: S, delay: Duration, scheduler: SC) -> Self {
        Debounce {
            delay: delay,
            scheduler: scheduler,
            stream: stream,
        }
    }
}

impl<S, SC, T> Stream<T> for Debounce<S, SC>
    where S: Stream<T> + 'static,
          SC: Scheduler + 'static
{
    fn consume<C>(self, consumer: C)
        where C: Consumer<T> + 'static
    {
        let consume = move || {
            self.stream.consume(DebounceState {
                delay: self.delay,
                mutex: Arc::new(Mutex::new(Some((consumer, self.scheduler)))),
                shared_item: Arc::new(Atom::empty()),
            })
        };

        if CurrentThread::current().running() {
            consume();
        } else {
            CurrentThread::current().schedule(consume, Duration::new(0, 0));
        }
    }
}

type MutexState<C, S> = Arc<Mutex<Option<(C, S)>>>;
type SharedItem<T> = Arc<Atom<Box<(Instant, T)>>>;

struct DebounceState<C, S, T> {
    delay: Duration,
    mutex: MutexState<C, S>,
    shared_item: SharedItem<T>,
}

fn emit<C, S, T>(mutex: MutexState<C, S>, item: T)
    where C: Consumer<T> + 'static
{
    let mut mutex = mutex.lock().unwrap();

    if let Some(ref mut consumer_scheduler) = *mutex {
        if consumer_scheduler.0.emit(item) {
            return;
        }
    }

    *mutex = None;
}

fn schedule<C, S, T>(mutex: MutexState<C, S>, shared_item: SharedItem<T>, delay: Duration) -> bool
    where C: Consumer<T> + 'static,
          S: Scheduler + 'static,
          T: 'static
{
    let cloned_mutex = mutex.clone();
    let cloned_shared_item = shared_item.clone();
    let mut mutex = mutex.lock().unwrap();

    if let Some(ref mut consumer_scheduler) = *mutex {

        let f = || {
            if let Some(instant_item) = cloned_shared_item.take() {
                let now = Instant::now();
                let due = instant_item.0;
                let item = instant_item.1;

                if due <= now {
                    emit(cloned_mutex, item);
                } else if cloned_shared_item.set_if_none(Box::new((due, item))).is_none() {
                    schedule(cloned_mutex, cloned_shared_item, due - now);
                }
            }
        };

        consumer_scheduler.1.schedule(f, delay);
        true
    } else {
        false
    }
}

impl<C, S, T> Consumer<T> for DebounceState<C, S, T>
    where C: Consumer<T> + 'static,
          S: Scheduler + 'static
{
    fn emit(&mut self, item: T) -> bool
        where T: 'static,
              C: 'static
    {
        let instant = Instant::now() + self.delay;

        if self.shared_item.swap(Box::new((instant, item))).is_none() {
            schedule(self.mutex.clone(), self.shared_item.clone(), self.delay)
        } else {
            true
        }
    }
}
