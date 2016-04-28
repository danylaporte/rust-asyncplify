use consumer::*;
use producer::*;
use std::rc::Rc;
use stream::*;

pub struct Value<T> {
    value: T,
}

pub struct ValueRef<T> {
    value: T,
}

pub struct ValueRefMut<T> {
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

impl<T> ValueRef<T> {
    pub fn new(value: T) -> Self {
        ValueRef { value: value }
    }
}

impl<T> ValueRefMut<T> {
    pub fn new(value: T) -> Self {
        ValueRefMut { value: value }
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

impl<T> StreamRef<T> for ValueRef<T> {
    fn consume<C: ConsumerRef<T>>(self, mut consumer: C) {
        let producer = Rc::new(Producer::new());

        consumer.init(producer.clone());

        if !producer.is_closed() {
            consumer.emit(&self.value);
        }
    }
}

impl<T> StreamRefMut<T> for ValueRefMut<T> {
    fn consume<C: ConsumerRefMut<T>>(mut self, mut consumer: C) {
        let producer = Rc::new(Producer::new());

        consumer.init(producer.clone());

        if !producer.is_closed() {
            consumer.emit(&mut self.value);
        }
    }
}