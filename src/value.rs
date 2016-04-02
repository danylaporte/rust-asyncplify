use std::rc::Rc;
use consumer::*;
use producer::*;
use stream::*;

pub struct Value<T> {
    value: T,
}

pub struct ValueRef<'a, T: 'a> {
    value: &'a T,
}

pub struct ValueRefMut<'a, T: 'a> {
    value: &'a mut T,
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

impl<'a, T> ValueRef<'a, T> {
    pub fn new(value: &'a T) -> Self {
        ValueRef { value: value }
    }
}

impl<'a, T> ValueRefMut<'a, T> {
    pub fn new(value: &'a mut T) -> Self {
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

        if !producer.is_closed() {
            consumer.end();
        }
    }
}

impl<'a, T> StreamRef<T> for ValueRef<'a, T> {
    fn consume<C: ConsumerRef<T>>(self, mut consumer: C) {
        let producer = Rc::new(Producer::new());

        consumer.init(producer.clone());

        if !producer.is_closed() {
            consumer.emit(self.value);
        }

        if !producer.is_closed() {
            consumer.end();
        }
    }
}

impl<'a, T> StreamRefMut<T> for ValueRefMut<'a, T> {
    fn consume<C: ConsumerRefMut<T>>(self, mut consumer: C) {
        let producer = Rc::new(Producer::new());

        consumer.init(producer.clone());

        if !producer.is_closed() {
            consumer.emit(self.value);
        }

        if !producer.is_closed() {
            consumer.end();
        }
    }
}