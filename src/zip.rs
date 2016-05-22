use consumer::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use stream::*;

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct Zip<L, R> {
    left: L,
    right: R,
}

impl<L, R> Zip<L, R>
{
    /// 'Zips up' two streams into a single stream of pairs.
    /// `zip()` returns a new stream that will iterate over two other streams, returning a tuple where the first element comes from the first stream,
    /// and the second element comes from the second stream.
    ///
    /// In other words, it zips two stream together, into a single one.
    ///
    /// # Examples
    /// 
    /// ```
    /// use asyncplify::*;
    ///
    /// let left = (0..4).to_stream();
    /// let right = (4..6).to_stream();
    ///
    /// let vec = Zip::new(left, right).into_vec();
    /// assert!(vec == [(0, 4),(1, 5)], "vec == {:?}", vec);
    /// ```
    pub fn new(left: L, right: R) -> Self {
        Zip {
            left: left,
            right: right,
        }
    }
}

impl<L, LS, R, RS> Stream<(L, R)> for Zip<LS, RS>
    where LS: Stream<L>,
          RS: Stream<R>
{
    fn consume<C: Consumer<(L, R)>>(self, consumer: C) {
        let common = Rc::new(RefCell::new(Common::new(consumer)));
        self.left.consume(ChildLeft { common: common.clone() });
        self.right.consume(ChildRight { common: common });
    }
}

struct ChildLeft<C, L, R> {
    common: Rc<RefCell<Common<C, L, R>>>,
}

struct ChildRight<C, L, R> {
    common: Rc<RefCell<Common<C, L, R>>>,
}

struct Common<C, L, R> {
    consumer: C,
    left: VecDeque<L>,
    left_closed: bool,
    right: VecDeque<R>,
    right_closed: bool,
}

impl<C, L, R> Common<C, L, R> {
    fn new(consumer: C) -> Self {
        Common {
            consumer: consumer,
            left: VecDeque::new(),
            left_closed: false,
            right: VecDeque::new(),
            right_closed: false,
        }
    }
}

impl<C, L, R> Consumer<L> for ChildLeft<C, L, R>
    where C: Consumer<(L, R)>
{
    fn emit(&mut self, item: L) -> bool {
        let ref mut common = *self.common.borrow_mut();

        if let Some(right) = common.right.pop_back() {
            common.consumer.emit((item, right)) && (!common.right_closed || common.right.len() > 0)
        } else if !common.right_closed {
            common.left.push_front(item);
            true
        } else {
            false
        }
    }
}

impl<C, L, R> Consumer<R> for ChildRight<C, L, R>
    where C: Consumer<(L, R)>
{
    fn emit(&mut self, item: R) -> bool {
        let ref mut common = *self.common.borrow_mut();

        if let Some(left) = common.left.pop_back() {
            common.consumer.emit((left, item)) && (!common.left_closed || common.left.len() > 0)
        } else if !common.left_closed {
            common.right.push_front(item);
            true
        } else {
            false
        }
    }
}
