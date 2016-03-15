use std::rc::Rc;
use consumer::*;
use producer::*;
use stream::*;

pub struct IterStream<I> {
    iterator: I,
}

impl<I> Stream for IterStream<I> where I: Iterator + 'static
{
    type Item = I::Item;

    fn consume<C>(self, mut consumer: C)
        where C: Consumer<Item = Self::Item>
    {
        let producer = Rc::new(Producer::new());

        consumer.init(producer.clone());

        if producer.is_closed() {
            return;
        }

        for i in self.iterator {
            consumer.emit(i);
            if producer.is_closed() {
                return;
            }
        }

        consumer.end();
    }
}

pub trait ToStream : Iterator {
    fn to_stream(self) -> IterStream<Self>
        where Self: Sized
    {
        IterStream { iterator: self }
    }
}

impl<I> ToStream for I where I: Iterator
{}
