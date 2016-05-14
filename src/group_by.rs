use consumer::*;
use producer::*;
use std::collections::hash_map::*;
use std::hash::Hash;
use std::marker::PhantomData;
use std::rc::Rc;
use std::cell::RefCell;
use stream::*;

pub struct Group<K, V> {
    consumer: Rc<RefCell<Option<Box<Consumer<V>>>>>,
    key: K,
}

impl<K, V> Stream<V> for Group<K, V>
    where V: 'static
{
    fn consume<C: Consumer<V> + 'static>(self, consumer: C) {
        *self.consumer.borrow_mut() = Some(Box::new(consumer));
    }
}

impl<K: Clone, V> Group<K, V> {
    fn new(key: K) -> Self {
        Group {
            consumer: Rc::new(RefCell::new(None)),
            key: key,
        }
    }

    pub fn get_key(&self) -> K {
        self.key.clone()
    }

    fn emit(&mut self, item: V) {
        if let Some(ref mut consumer) = *self.consumer.borrow_mut() {
           consumer.emit(item);
        }
    }
    
    fn clone(&self) -> Self {
        Group {
            consumer: self.consumer.clone(),
            key: self.key.clone(), 
        }
    }
}

pub struct GroupBy<C, F, K, V> {
    consumer: C,
    hashmap: HashMap<K, Group<K, V>>,
    key_selector: F,
}

impl<C, F, K, V> Consumer<V> for GroupBy<C, F, K, V>
    where C: Consumer<Group<K, V>>,
          F: FnMut(&V) -> K,
          K: Hash + Eq + Clone
{
    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: V) {
        let key = (self.key_selector)(&item);
        let consumer = &mut self.consumer;
        let group = self.hashmap
                        .entry(key.clone())
                        .or_insert_with(|| {
                            let g = Group::new(key);
                            consumer.emit(g.clone());
                            g
                        });

        group.emit(item);
    }
}

pub struct GroupByFactory<F, K, S, V> {
    key_selector: F,
    marker_k: PhantomData<K>,
    marker_v: PhantomData<V>,
    stream: S,
}

impl<F, K, S, V> Stream<Group<K, V>> for GroupByFactory<F, K, S, V>
    where F: FnMut(&V) -> K,
          K: Clone + Hash + Eq,
          S: Stream<V>
{
    fn consume<C: Consumer<Group<K, V>>>(self, consumer: C) {
        self.stream.consume(GroupBy {
            consumer: consumer,
            hashmap: HashMap::new(),
            key_selector: self.key_selector,
        });
    }
}

pub trait GroupByStream<T>: Stream<T> {
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
    fn group_by<F: FnMut(&V) -> K, K, V>(self, key_selector: F) -> GroupByFactory<F, K, Self, V>
        where Self: Sized
    {
        GroupByFactory {
            key_selector: key_selector,
            marker_k: PhantomData::<K>,
            marker_v: PhantomData::<V>,
            stream: self,
        }
    }
}

impl<S, T> GroupByStream<T> for S where S: Stream<T> {}