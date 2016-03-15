use std::rc::Rc;
use consumer::*;
use producer::*;
use stream::*;

pub struct Value<T> {
    value: T,
}

impl<T> Value<T> {
    /// Constructs a new `Stream` based on a `Value<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    ///
    /// Value::new(5)
    ///     .tap(|v| println!("{}", v))
    ///     .subscribe();
    /// ```
    pub fn new(value: T) -> Self {
        Value { value: value }
    }
}

impl<T> Stream for Value<T>
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