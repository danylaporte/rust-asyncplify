use std::rc::Rc;
use consumer::*;
use producer::*;
use stream::*;
use unit::*;

/// Constructs a new empty `Stream` based on a `Empty`.
///
/// # Examples
///
/// ```
/// use asyncplify::*;
///
/// Empty
///     .subscribe();
/// ```
pub struct Empty;

impl Stream for Empty {
    type Item = Unit;

    fn consume<C>(self, mut consumer: C)
        where C: Consumer<Item = Self::Item>
    {
        let producer = Rc::new(Producer::new());

        consumer.init(producer.clone());

        if !producer.is_closed() {
            consumer.end();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use subscription::*;
    use tap::*;

    #[test]
    fn it_works() {
        let mut count = 0;

        Empty.tap(|_| count += 1)
             .subscribe();

        assert!(count == 0, "count = {}", count);
    }
}
