use consumer::*;
use observe_on::*;
use super::schedulers::ParallelScheduler;

pub trait ParallelStream<T: Send>: Send {
    fn consume<C: ParallelConsumer<T>>(self, C);

    fn observe_on<SC>(self, scheduler: SC) -> ParallelObserveOn<Self, SC>
        where SC: ParallelScheduler,
              Self: Sized
    {
        ParallelObserveOn::new(self, scheduler)
    }
}