use consumer::*;
use producer::*;
use std::rc::Rc;
use stream::*;

pub struct IterStream<I> {
    iterator: I,
}

pub struct IterStreamRef<I> {
    iterator: I,
}

pub struct IterStreamRefMut<I> {
    iterator: I,
}

impl<I, T> Stream<T> for IterStream<I>
    where I: Iterator<Item = T> + 'static
{
    fn consume<C: Consumer<T>>(self, mut consumer: C) {
        let producer = Rc::new(Producer::new());

        consumer.init(producer.clone());

        if producer.is_closed() {
            return;
        }

        for i in self.iterator {
            consumer.emit(i);
            if producer.is_closed() {
                return;
            }
        }
    }
}

impl<I, T> StreamRef<T> for IterStreamRef<I>
    where I: Iterator<Item = T> + 'static
{
    fn consume<C: ConsumerRef<T>>(self, mut consumer: C) {
        let producer = Rc::new(Producer::new());

        consumer.init(producer.clone());

        if producer.is_closed() {
            return;
        }

        for i in self.iterator {
            consumer.emit(&i);
            if producer.is_closed() {
                return;
            }
        }
    }
}

impl<I, T> StreamRefMut<T> for IterStreamRefMut<I>
    where I: Iterator<Item = T> + 'static
{
    fn consume<C: ConsumerRefMut<T>>(self, mut consumer: C) {
        let producer = Rc::new(Producer::new());

        consumer.init(producer.clone());

        if producer.is_closed() {
            return;
        }

        for mut i in self.iterator {
            consumer.emit(&mut i);
            if producer.is_closed() {
                return;
            }
        }
    }
}

pub trait ToStream: Iterator {
    /// Convert an iterator to a `Stream`.
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    ///
    /// let mut sum = 0;
    ///
    /// (0..5)
    ///     .to_stream()
    ///     .tap(|v| sum += *v)
    ///     .subscribe();
    ///
    /// assert!(sum == 10, "sum (0 + 1 + 2 + 3 + 4) = {}", sum);
    /// ``` 
    fn to_stream(self) -> IterStream<Self>
        where Self: Sized
    {
        IterStream { iterator: self }
    }

    fn to_stream_ref(self) -> IterStreamRef<Self>
        where Self: Sized
    {
        IterStreamRef { iterator: self }
    }

    fn to_stream_ref_mut(self) -> IterStreamRefMut<Self>
        where Self: Sized
    {
        IterStreamRefMut { iterator: self }
    }
}

impl<I> ToStream for I where I: Iterator {}
