use consumer::*;
use producer::*;
use std::rc::Rc;
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
    ///     .inspect(|v| println!("{}", v))
    ///     .subscribe();
    /// ```
    pub fn new(value: T) -> Self {
        Value { value: value }
    }
}

impl<T> Stream<T> for Value<T> {
    fn consume<C: Consumer<T>>(self, mut consumer: C) {
        let producer = Rc::new(Producer::new());

        consumer.init(producer.clone());

        if !producer.is_closed() {
            consumer.emit(self.value);
        }
    }
}