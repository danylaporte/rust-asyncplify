use consumer::*;
use count::*;
use filter::*;
use inspect::*;
use max::*;
use max_by_key::*;
use min::*;
use min_by_key::*;
use skip::*;
use skip_last::*;
use sum::*;
use take::*;
use take_last::*;

pub trait Stream<T> {
    fn consume<C: Consumer<T>>(self, consumer: C);

    /// Count the number ofthe item received.
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    ///
    /// let vec = (0..10)
    ///     .to_stream()
    ///     .count()
    ///     .into_vec();
    /// assert!(vec == [10], "vec = {:?}", vec);
    /// ```
    fn count(self) -> Count<Self, T>
        where Self: Sized
    {
        Count::new(self)
    }

    /// Filter a `Stream` based on a predicate `Stream`.
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    ///
    /// let mut vec = Vec::new();
    ///
    /// (0..5)
    ///     .to_stream()
    ///     .filter(|v| *v > 2)
    ///     .inspect(|v| vec.push(*v))
    ///     .subscribe();
    ///
    /// assert!(vec == &[3, 4], "vec = {:?}", vec);
    /// ``` 
    fn filter<F>(self, predicate: F) -> Filter<Self, F>
        where Self: Sized,
              F: FnMut(&mut T) -> bool
    {
        Filter::new(self, predicate)
    }

    /// Do something with each element of a stream, passing the value on.
    /// This is usefull to debug an item.
    fn inspect<F>(self, func: F) -> Inspect<Self, F>
        where F: FnMut(&mut T),
              Self: Sized
    {
        Inspect::new(self, func)
    }

    /// Emit the item corresponding to the maximum value.
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    /// let mut value = 0;
    ///
    /// (0..10)
    ///     .to_stream()
    ///     .max()
    ///     .inspect(|v| value = *v)
    ///     .subscribe();
    /// assert!(value == 9, "value = {:?}", value);
    /// ```
    fn max(self) -> Max<Self>
        where Self: Sized
    {
        Max::new(self)
    }

    /// Emit the item corresponding to the maximum value.
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    /// let mut value = 100;
    ///
    /// (0..10)
    ///     .to_stream()
    ///     .max_by_key(|v| 10 - *v)
    ///     .inspect(|v| value = *v)
    ///     .subscribe();
    /// assert!(value == 0, "value = {:?}", value);
    /// ```
    fn max_by_key<F: FnMut(&T) -> K, K>(self, f: F) -> MaxByKey<Self, F, K>
        where Self: Sized
    {
        MaxByKey::new(self, f)
    }

    /// Emit the item corresponding to the minimum value.
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    /// let mut value = 100;
    ///
    /// (0..10)
    ///     .to_stream()
    ///     .min()
    ///     .inspect(|v| value = *v)
    ///     .subscribe();
    /// assert!(value == 0, "value = {:?}", value);
    /// ```
    fn min(self) -> Min<Self>
        where Self: Sized
    {
        Min::new(self)
    }

    /// Emit the item corresponding to the minimum value.
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    /// let mut value = 100;
    ///
    /// (0..10)
    ///     .to_stream()
    ///     .min_by_key(|v| 10 - *v)
    ///     .inspect(|v| value = *v)
    ///     .subscribe();
    /// assert!(value == 9, "value = {:?}", value);
    /// ```
    fn min_by_key<F: FnMut(&T) -> K, K>(self, f: F) -> MinByKey<Self, F, K>
        where Self: Sized
    {
        MinByKey::new(self, f)
    }

    /// Ignore the first X values from the stream
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    ///
    /// let vec = (0..10)
    ///     .to_stream()
    ///     .skip(3)
    ///     .into_vec();
    /// assert!(vec == [3, 4, 5, 6, 7, 8, 9], "vec = {:?}", vec);
    /// ```
    fn skip(self, count: u64) -> Skip<Self>
        where Self: Sized
    {
        Skip::new(self, count)
    }

    /// Ignores the last X values of the stream
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    ///
    /// let vec = (0..10)
    ///     .to_stream()
    ///     .skip_last(3)
    ///     .into_vec();
    /// assert!(vec == [0, 1, 2, 3, 4, 5, 6], "vec = {:?}", vec);
    /// ```
    fn skip_last(self, count: usize) -> SkipLast<Self>
        where Self: Sized
    {
        SkipLast::new(self, count)
    }

    /// Calculate the sum of the item received.
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    /// let vec = (0..10)
    ///     .to_stream()
    ///     .sum()
    ///     .into_vec();
    /// assert!(vec == [45], "vec = {:?}", vec);
    /// ```
    fn sum(self) -> Sum<Self>
        where Self: Sized
    {
        Sum::new(self)
    }

    /// Take only the first X values of the stream and close the stream after
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    ///
    /// let vec = (0..10)
    ///     .to_stream()
    ///     .take(3)
    ///     .into_vec();
    /// assert!(vec == [0, 1, 2], "vec = {:?}", vec);
    /// ```
    fn take(self, count: u64) -> Take<Self>
        where Self: Sized
    {
        Take::new(self, count)
    }

    /// Take the only the last X values of the stream and close the stream after
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    ///
    /// let vec = (0..10)
    ///     .to_stream()
    ///     .take_last(3)
    ///     .into_vec();
    /// assert!(vec == [7, 8, 9], "vec = {:?}", vec);
    /// ```
    fn take_last(self, count: usize) -> TakeLast<Self>
        where Self: Sized
    {
        TakeLast::new(self, count)
    }
}