use consumer::*;
use stream::*;
use parallel_stream::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
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

pub struct SyncToParallelObserveOn<S, SC> {
    scheduler: SC,
    stream: S,
}

impl<S, SC> SyncToParallelObserveOn<S, SC> {
    pub fn new(stream: S, scheduler: SC) -> Self {
        SyncToParallelObserveOn {
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
        self.stream.consume(ParallelObserveOnState::new(consumer, self.scheduler));
    }
}

impl<S, SC, T> ParallelStream<T> for SyncToParallelObserveOn<S, SC>
    where S: Stream<T>,
          SC: ParallelScheduler,
          T: Send + 'static
{
    fn consume<C>(self, consumer: C)
        where C: ParallelConsumer<T> + 'static
    {
        self.stream.consume(ParallelObserveOnState::new(consumer, self.scheduler));
    }
}

struct ParallelObserveOnState<C, S> {
    completed_consumer: Arc<(AtomicBool, C)>,
    scheduler: S,
}

impl<C, S> ParallelObserveOnState<C, S> {
    fn new(consumer: C, scheduler: S) -> Self {
        ParallelObserveOnState {
            completed_consumer: Arc::new((AtomicBool::new(false), consumer)),
            scheduler: scheduler,
        }
    }

    fn schedule_emit<T>(&self, item: T) -> bool
        where T: Send + 'static,
              C: ParallelConsumer<T> + 'static,
              S: ParallelScheduler
    {
        let completed_consumer = self.completed_consumer.clone();

        if completed_consumer.0.load(Ordering::Relaxed) {
            return false;
        }

        let f = move || {
            if !completed_consumer.0.load(Ordering::Relaxed) {
                if !completed_consumer.1.emit(item) {
                    completed_consumer.0.store(false, Ordering::SeqCst);
                }
            }
        };

        self.scheduler.schedule(f, Duration::new(0, 0));
        true
    }
}

impl<C, S, T> Consumer<T> for ParallelObserveOnState<C, S>
    where T: Send + 'static,
          C: ParallelConsumer<T> + 'static,
          S: ParallelScheduler
{
    fn emit(&mut self, item: T) -> bool {
        self.schedule_emit(item)
    }
}

impl<C, S, T> ParallelConsumer<T> for ParallelObserveOnState<C, S>
    where T: Send + 'static,
          C: ParallelConsumer<T> + 'static,
          S: ParallelScheduler
{
    fn emit(&self, item: T) -> bool {
        self.schedule_emit(item)
    }
}
