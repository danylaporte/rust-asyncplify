use std::collections::hash_map::*;
use std::cell::*;
use std::hash::Hash;
use std::marker::PhantomData;
use std::mem;
use std::ops::{Add, Deref};
use std::rc::Rc;
use consumer::*;
use producer::*;
use stream::*;

struct Group<K, T> {
    consumer: Option<Box<Consumer<Item = T>>>,
    key: K,
}

impl<K, V> Group<K, V> {
    fn emit(&mut self, item: V) {
        if let Some(ref mut c) = self.consumer {
            c.emit(item);
        }
    }

    fn end(&mut self) {
        if let Some(mut c) = mem::replace(&mut self.consumer, None) {
            c.end();
        }
    }

    fn new(key: K) -> Self {
        Group {
            consumer: None,
            key: key,
        }
    }
}

struct GroupByState<C, F, K, T> {
    consumer: C,
    func: F,
    hashmap: HashMap<K, Rc<Group<K, T>>>,
}

impl<C, F, K, T> Consumer for GroupByState<C, F, K, T>
    where C: Consumer<Item = Rc<Group<K, T>>>,
          F: FnMut(&T) -> K,
          K: Eq + Hash + Clone
{
    type Item = T;

    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: Self::Item) {
        let key = (self.func)(&item);
        
        if let Some(g) = self.hashmap.get(&key) {
            g.borrow_mut().emit(item);
            return;
        }

        self.hashmap.insert(key.clone(), Rc::new(RefCell::new(Group::new(key.clone())));
        let g = self.hashmap.get(&key).unwrap();
        self.consumer.emit(g.borrow());
        g.borrow_mut().emit(item);
    }

    fn end(&mut self) {
        for kv in self.hashmap.drain() {
            kv.1.borrow_mut().end();
        }

        self.consumer.end();
    }
}

pub struct GroupBy<S, F, K> {
    stream: S,
    func: F,
    marker_k: PhantomData<K>,
}

impl<S, F, K> Stream for GroupBy<S, F, K>
    where S: Stream,
          F: FnMut(&<S as Stream>::Item) -> K,
          K: Eq + Hash + Clone
{
    type Item = Group<K, S::Item>;

    fn consume<C>(self, consumer: C)
        where C: Consumer<Item = Self::Item>
    {
        self.stream.consume(GroupByState {
            consumer: consumer,
            func: self.func,
            hashmap: HashMap::new(),
        });
    }
}

impl<K, V> Stream for Group<K, V> {
    type Item = V;

    fn consume<C>(mut self, consumer: C)
        where C: Consumer<Item = Self::Item>
    {
        self.consumer = Some(Box::new(consumer));
    }
}

pub trait GroupByStream: Stream {
    fn group_by<F, K>(self, func: F) -> GroupBy<Self, F, K>
        where Self: Sized,
              F: FnMut(&Self::Item) -> K
    {
        GroupBy {
            stream: self,
            func: func,
            marker_k: PhantomData::<K>,
        }
    }
}

impl<S> GroupByStream for S where S: Stream {}