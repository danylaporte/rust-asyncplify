use consumer::*;
use parallel_stream::*;
use std::sync::mpsc::{Sender, channel};
use std::sync::Mutex;
use std::time::Duration;
use super::schedulers::ParallelScheduler;

pub struct ParallelObserveOn<S, SC> {
    scheduler: SC,
    stream: S,
}

impl<S, SC> ParallelObserveOn<S, SC> {
    pub fn new(stream: S, scheduler: SC) -> Self {
        ParallelObserveOn {
            scheduler: scheduler,
            stream: stream,
        }
    }
}

impl<S, SC, T> ParallelStream<T> for ParallelObserveOn<S, SC>
    where S: ParallelStream<T>,
          SC: ParallelScheduler,
          T: Send + 'static
{
    fn consume<C>(self, consumer: C)
        where C: ParallelConsumer<T> + 'static
    {
        let mutex = Mutex::new(consumer);
        let (tx, rc) = channel();

        let f = move || {
            let consumer = mutex.lock().unwrap();

            while let Ok(item) = rc.recv() {
                if !consumer.emit(item) {
                    break;
                }
            }
        };

        self.scheduler.schedule(f, Duration::new(0, 0));
        self.stream.consume(ParallelObserveOnState { tx: tx });
    }
}

struct ParallelObserveOnState<T> {
    tx: Sender<T>,
}

impl<T> ParallelConsumer<T> for ParallelObserveOnState<T>
    where T: Send
{
    fn emit(&self, item: T) -> bool {
        self.tx.send(item).is_ok()
    }
}