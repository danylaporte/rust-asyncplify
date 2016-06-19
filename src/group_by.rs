use consumer::*;
use std::cell::RefCell;
use std::collections::hash_map::*;
use std::hash::Hash;
use std::rc::Rc;
use stream::*;

pub struct Group<K, V> {
    consumer: Rc<RefCell<Option<Box<Consumer<V>>>>>,
    key: K,
}

impl<K, V> Stream for Group<K, V>
    where V: 'static
{
    type Item = V;

    fn consume<C>(self, consumer: C)
        where C: Consumer<Self::Item> + 'static
    {
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
        let mut consumer_ref = self.consumer.borrow_mut();
        {
            if let Some(ref mut consumer) = *consumer_ref {
                if consumer.emit(item) {
                    return;
                }
            }
        }
        *consumer_ref = None;
    }
}

impl<K: Clone, V> Clone for Group<K, V> {
    fn clone(&self) -> Self {
        Group {
            consumer: self.consumer.clone(),
            key: self.key.clone(),
        }
    }
}

struct GroupByState<C, F, K, V> {
    consumer: C,
    hashmap: HashMap<K, Group<K, V>>,
    key_selector: F,
}

impl<C, F, K, V> Consumer<V> for GroupByState<C, F, K, V>
    where C: Consumer<Group<K, V>>,
          F: FnMut(&V) -> K,
          K: Hash + Eq + Clone
{
    fn emit(&mut self, item: V) -> bool {
        let key = (self.key_selector)(&item);
        let consumer = &mut self.consumer;
        let mut is_available = true;
        let mut g = self.hashmap
            .entry(key.clone())
            .or_insert_with(|| {
                let g = Group::new(key);
                is_available = consumer.emit(g.clone());
                g
            });

        if is_available {
            g.emit(item);
        }

        is_available
    }
}

#[must_use = "stream adaptors are lazy and do nothing unless consumed"]
pub struct GroupBy<F, S> {
    key_selector: F,
    stream: S,
}

impl<F, S> GroupBy<F, S> {
    pub fn new(stream: S, key_selector: F) -> Self {
        GroupBy {
            key_selector: key_selector,
            stream: stream,
        }
    }
}

impl<F, K, S> Stream for GroupBy<F, S>
    where S: Stream,
          F: FnMut(&S::Item) -> K,
          K: Clone + Hash + Eq
{
    type Item = Group<K, S::Item>;

    fn consume<C>(self, consumer: C)
        where C: Consumer<Group<K, S::Item>>
    {
        self.stream.consume(GroupByState {
            consumer: consumer,
            hashmap: HashMap::new(),
            key_selector: self.key_selector,
        });
    }
}
