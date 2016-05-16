use consumer::*;
use stream::*;

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
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
        consumer.emit(self.value);
    }
}