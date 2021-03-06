use consumer::*;
use parallel_stream::*;
use std::cmp::PartialOrd;
use std::sync::Mutex;
use stream::*;

struct MaxByKeyState<C, T, F, K>
    where C: Consumer<T>,
          F: FnMut(&T) -> K,
          K: PartialOrd
{
    consumer: C,
    f: F,
    value: Option<(K, T)>,
}

impl<C, T, F, K> Consumer<T> for MaxByKeyState<C, T, F, K>
    where C: Consumer<T>,
          F: FnMut(&T) -> K,
          K: PartialOrd
{
    fn emit(&mut self, item: T) -> bool {

        let mut value = ((self.f)(&item), item);

        if let Some(current) = self.value.take() {
            if current.0 > value.0 {
                value = current;
            }
        }

        self.value = Some(value);
        true
    }
}

impl<C, T, F, K> Drop for MaxByKeyState<C, T, F, K>
    where C: Consumer<T>,
          F: FnMut(&T) -> K,
          K: PartialOrd
{
    fn drop(&mut self) {
        if let Some(value) = self.value.take() {
            self.consumer.emit(value.1);
        }
    }
}

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct MaxByKey<S, F> {
    f: F,
    stream: S,
}

impl<S, F, K> Stream for MaxByKey<S, F>
    where S: Stream,
          F: FnMut(&S::Item) -> K,
          K: PartialOrd
{
    type Item = S::Item;

    fn consume<C>(self, consumer: C)
        where C: Consumer<Self::Item>
    {
        self.stream.consume(MaxByKeyState {
            consumer: consumer,
            f: self.f,
            value: None,
        });
    }
}

impl<S, F> MaxByKey<S, F> {
    pub fn new(stream: S, f: F) -> Self {
        MaxByKey {
            f: f,
            stream: stream,
        }
    }
}

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct ParallelMaxByKey<S, F> {
    f: F,
    stream: S,
}

impl<S, F> ParallelMaxByKey<S, F> {
    pub fn new(stream: S, f: F) -> Self {
        ParallelMaxByKey {
            f: f,
            stream: stream,
        }
    }
}

impl<S, F, K> ParallelStream for ParallelMaxByKey<S, F>
    where S: ParallelStream,
          F: Send + Sync + Fn(&S::Item) -> K,
          K: PartialOrd + Send,
          S::Item: Send
{
    type Item = S::Item;

    fn consume<C>(self, consumer: C)
        where C: ParallelConsumer<Self::Item>
    {
        self.stream.consume(ParallelMaxByKeyState {
            consumer: consumer,
            f: self.f,
            value: Mutex::new(None),
        });
    }
}

struct ParallelMaxByKeyState<C, T, F, K>
    where C: ParallelConsumer<T>,
          F: Send + Sync + Fn(&T) -> K,
          K: PartialOrd + Send,
          T: Send
{
    consumer: C,
    f: F,
    value: Mutex<Option<(K, T)>>,
}

impl<C, T, F, K> ParallelConsumer<T> for ParallelMaxByKeyState<C, T, F, K>
    where C: ParallelConsumer<T>,
          F: Send + Sync + Fn(&T) -> K,
          K: PartialOrd + Send,
          T: Send
{
    fn emit(&self, item: T) -> bool {

        let mut value = ((self.f)(&item), item);
        let mut lock = self.value.lock().unwrap();

        if let Some(current) = lock.take() {
            if current.0 < value.0 {
                value = current;
            }
        }

        *lock = Some(value);
        true
    }
}

impl<C, T, F, K> Drop for ParallelMaxByKeyState<C, T, F, K>
    where C: ParallelConsumer<T>,
          F: Send + Sync + Fn(&T) -> K,
          K: PartialOrd + Send,
          T: Send
{
    fn drop(&mut self) {
        if let Some(value) = self.value.lock().unwrap().take() {
            self.consumer.emit(value.1);
        }
    }
}