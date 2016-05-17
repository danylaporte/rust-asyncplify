use consumer::*;
use count::*;
use filter::*;
use flat_map::*;
use fold::*;
use group_by::*;
use inspect::*;
use map::*;
use max_by_key::*;
use max::*;
use min_by_key::*;
use min::*;
use scan::*;
use skip_last::*;
use skip::*;
use subscription::*;
use sum::*;
use take_last::*;
use take::*;
use unique::*;
use zip::*;

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

    /// Creates a stream that emit only new elements. If an element has already been emitted, it is ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    ///
    /// let vec = [0, 1, 1, 0, 2, 3]
    ///     .into_iter()
    ///     .map(|i| *i)
    ///     .to_stream()
    ///     .unique()
    ///     .into_vec();
    ///
    /// assert!(vec == [0, 1, 2, 3], "vec = {:?}", vec);
    /// ```     
    fn unique(self) -> Unique<Self>
        where Self: Sized
    {
        Unique::new(self)
    }

    /// Creates a stream which uses a closure to determine if an element should be emitted.
    /// The closure must return true or false. `filter()` creates a stream which calls this closure on each element. If the closure returns true, 
    /// then the element is returned. If the closure returns false, it will try again, and call the closure on the next element, seeing if it passes the test.
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

    /// Creates an stream that works like map, but flattens nested structure.
    /// The `map()` adapter is very useful, but only when the closure argument produces values. 
    /// If it produces a stream instead, there's an extra layer of indirection. flat_map() will remove this extra layer on its own.
    ///
    /// Another way of thinking about flat_map(): map()'s closure returns one item for each element, and flat_map()'s closure returns a stream for each element.
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    ///
    /// let vec = (0..4i32)
    ///     .to_stream()
    ///     .flat_map(|v| Value::new(v + 10))
    ///     .into_vec();
    ///
    /// assert!(vec == [10, 11, 12, 13], "vec = {:?}", vec);
    /// ```
    fn flat_map<F, SO, O>(self, func: F) -> Flatmap<Self, F, T, SO, O>
        where Self: Sized,
              F: FnMut(T) -> SO,
              SO: Stream<O>
    {
        Flatmap::new(self, func)
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
    /// # Examples
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

    /// Takes a closure and creates a stream which calls that closure on each element.
    /// `map()` transforms one stream into another, by means of its argument: something that implements FnMut. It produces a new stream which calls this closure 
    /// on each element of the original stream.
    ///
    /// If you are good at thinking in types, you can think of map() like this: If you have a stream that gives you elements of some type A, and you want a stream
    /// of some other type B, you can use `map()`, passing a closure that takes an A and returns a B.
    /// `map()` is conceptually similar to a for loop. However, as `map()` is lazy, it is best used when you're already working with other streams. If you're doing
    /// some sort of looping for a side effect, it's considered more idiomatic to use for than `map()`.`
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    ///
    /// let mut value = 0;
    ///
    /// let vec = (0..4)
    ///     .to_stream()
    ///     .map(|v| v + 10)
    ///     .into_vec();
    /// assert!(vec == [10, 11, 12, 13], "vec = {:?}", vec);
    /// ```
    fn map<O, F>(self, func: F) -> Map<Self, F, T, O>
        where Self: Sized,
              F: FnMut(T) -> O
    {
        Map::new(self, func)
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

    /// A stream adaptor similar to `fold()` that holds internal state and produces a new stream.
    /// `scan()` takes two arguments: an initial value which seeds the internal state, and a closure with two arguments, 
    /// the first being a mutable reference to the internal state and the second an stream element. The closure can assign 
    /// to the internal state to share state between iterations.

    /// On iteration, the closure will be applied to each element of the stream and the return value from the closure, an Option, is emitted by the stream.
    ///
    /// # Examples
    ///
    /// ```
    /// use asyncplify::*;
    ///
    /// let vec = (0..6)
    ///     .to_stream()
    ///     .scan(0, |o, i| o + i)
    ///     .into_vec();
    ///
    /// assert!(vec == [0, 1, 3, 6, 10, 15], "vec = {:?}", vec);
    /// ```
    fn scan<O, F>(self, initial: O, func: F) -> Scan<Self, T, F, O>
        where Self: Sized,
              F: FnMut(O, T) -> O,
              O: Clone
    {
        Scan::new(self, initial, func)
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

    fn subscribe(self)
        where Self: Sized
    {
        self.consume(Subscription::new());
    }

    /// Sums the elements of a stream.
    /// Takes each element, adds them together, and returns the result.
    /// An empty stream returns the zero value of the type.
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
    /// let right = (4..6).to_stream();
    ///
    /// let vec = (0..4).to_stream().zip(right).into_vec();
    /// assert!(vec == [(0, 4),(1, 5)], "vec == {:?}", vec);
    /// ```
    fn zip<R>(self, right: R) -> Zip<Self, R>
        where Self: Sized
    {
        Zip::new(self, right)
    }
}