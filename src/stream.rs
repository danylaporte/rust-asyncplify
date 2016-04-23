use consumer::*;

pub trait Stream<T> {
    fn consume<C: Consumer<T>>(self, consumer: C);
}

pub trait StreamRef<T> {
    fn consume<C: ConsumerRef<T>>(self, consumer: C);
}

pub trait StreamRefMut<T> {
    fn consume<C: ConsumerRefMut<T>>(self, consumer: C);
}