use std::rc::Rc;
use consumer::*;
use producer::*;
use stream::*;

pub struct ValueStream<T> {
    value: T,
}

impl<T> ValueStream<T> {
    /// Constructs a new `ValueStream<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    ///
    /// ValueStream::new(5)
    ///     .tap(|v| println!("{}", v))
    ///     .subscribe();
    /// ```
    pub fn new(value: T) -> Self {
        ValueStream { value: value }
    }
}

impl<T> Stream for ValueStream<T>
    where T: Clone
{
    type Item = T;

    fn consume<C>(self, mut consumer: C)
        where C: Consumer<Item = Self::Item>
    {
        let producer = Rc::new(Producer::new());
        
        consumer.init(producer.clone());
        
        if !producer.is_closed() {
            consumer.emit(self.value.clone());
        }
        
        if !producer.is_closed() {
            consumer.end();
        }
    }
}