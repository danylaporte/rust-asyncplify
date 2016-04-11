use std::rc::Rc;
use consumer::*;
use producer::*;
use stream::*;

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

impl Stream<()> for Empty {
    fn consume(self, consumer: &mut Consumer<()>) {
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
