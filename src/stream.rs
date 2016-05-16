use consumer::*;
use count::*;
use filter::*;
use fold::*;
use group_by::*;
use inspect::*;
use max_by_key::*;
use max::*;
use min_by_key::*;
use min::*;
use skip_last::*;
use skip::*;
use sum::*;
use take_last::*;
use take::*;

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

    /// A stream adaptor that applies a function, producing a single, final value.
    ///`fold()` takes two arguments: an initial value, and a closure with two arguments: an 'accumulator', and an element. 
    /// It returns the value that the accumulator should have for the next iteration.
    ///
    /// The initial value is the value the accumulator will have on the first call.
    /// After applying this closure to every element of the iterator, `fold()` returns the accumulator.
    ///
    /// This operation is sometimes called 'reduce' or 'inject'.

    /// Folding is useful whenever you have a collection of something, and want to produce a single value from it.
    ///
    /// # Basic example
    ///
    /// ```
    /// use asyncplify::*;
    /// let mut v = 0;
    ///
    /// (0..10)
    ///     .to_stream()
    ///     .fold(0, |o, i| o + i)
    ///     .inspect(|x| v = *x)
    ///     .subscribe();
    ///
    /// assert!(v == 45, "v = {}", v);
    /// ```
    fn fold<O, F>(self, initial: O, func: F) -> Fold<Self, T, F, O>
        where Self: Sized,
              F: FnMut(O, T) -> O
    {
        Fold::new(self, initial, func)
    }

    /// Group incoming values using a `key_selector`.
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    ///
    /// let mut vec = Vec::new();
    ///
    /// (0..10)
    ///     .to_stream()
    ///     .group_by(|v| v % 2)
    ///     .inspect(|g| vec.push(g.get_key()))
    ///     .subscribe();
    ///
    /// // This gives 2 groups
    /// assert!(vec == vec!(0, 1), "vec = {:?}", vec);
    /// ```
    fn group_by<F: FnMut(&V) -> K, K, V>(self, key_selector: F) -> GroupBy<F, K, Self, V>
        where Self: Sized
    {
        GroupBy::new(self, key_selector)
    }

    /// Do something with each element of a stream, passing the value on.
    /// This is usefull to debug an item.
    fn inspect<F>(self, func: F) -> Inspect<Self, F>
        where F: FnMut(&mut T),
              Self: Sized
    {
        Inspect::new(self, func)
    }

    /// Returns the maximum element of a stream.
    /// Returns the lastest element if the comparison determines two elements to be equally maximum.
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

    /// Returns the element that gives the maximum value from the specified function.
    /// Returns the lastest element if the comparison determines two elements to be equally maximum.
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

    /// Returns the minimum element of a stream.
    /// Returns the lastest element if the comparison determines two elements to be equally minimum.
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

    /// Returns the element that gives the minimum value from the specified function.
    /// Returns the lastest element if the comparison determines two elements to be equally minimum.
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