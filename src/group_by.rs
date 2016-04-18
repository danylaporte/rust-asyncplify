use consumer::*;
use producer::*;
use std::cell::*;
use std::collections::hash_map::*;
use std::hash::Hash;
use std::marker::PhantomData;
use std::mem;
use std::ops::{Add, Deref};
use std::rc::Rc;
use stream::*;

struct Group<'a, K, T: 'a> {
    consumer: Option<&'a mut Consumer<T>>,
    key: K,
}

impl<'a, K, V> Group<'a, K, V> {
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

struct GroupByState<'a, F, K: 'a, T: 'a> {
    consumer: &'a mut ConsumerRefMut<Group<'a, K, T>>,
    hashmap: HashMap<K, Group<'a, K, T>>,
    key_selector: F,
}

impl<'a, F: FnMut(&T) -> K, K: Eq + Hash + Copy, T> Consumer<T> for GroupByState<'a, F, K, T> {
    fn init(&mut self, producer: Rc<Producer>) {
        self.consumer.init(producer);
    }

    fn emit(&mut self, item: T) {
        let key = (self.key_selector)(&item);
        let consumer = &mut self.consumer;

        let entry = self.hashmap.entry(key).or_insert_with(|| {
            let mut group = Group {
                consumer: None,
                key: key,
            };

            consumer.emit(&mut group);
            group
        });

        entry.emit(item);
    }

    fn end(&mut self) {
        for kv in self.hashmap.drain() {
            kv.1.end();
        }

        self.consumer.end();
    }
}

pub struct GroupBy<S, F, K, V> {
    key_selector: F,
    marker_K: PhantomData<K>,
    marker_V: PhantomData<V>,
    stream: S,
}

impl<'a, S: Stream<V>, F: FnMut(&V) -> K, K: Eq + Hash + Copy, V> StreamRefMut<Group<'a, K, V>> for GroupBy<S,F,K,V> {
    fn consume(self, consumer: &mut ConsumerRefMut<Group<'a, K, V>>) {
        self.stream.consume(&mut GroupByState {
            consumer: consumer,
            hashmap: HashMap::new(),
            key_selector: self.key_selector,
        });
    }
}

impl<'a, K, V> Stream<V> for Group<'a, K, V> {
    fn consume(mut self, consumer: &mut Consumer<V>) {
        self.consumer = Some(consumer);
    }
}

pub trait GroupByStream<T>: StreamRefMut<T> {
    fn group_by<F: FnMut(&V) -> K, K, V>(self, key_selector: F) -> GroupBy<Self, F, K, V>
        where Self: Sized
    {
        GroupBy {
            key_selector: key_selector,
            marker_K: PhantomData::<K>,
            marker_V: PhantomData::<V>,
            stream: self,
        }
    }
}

impl<S, T> GroupByStream<T> for S where S: StreamRefMut<T> {}