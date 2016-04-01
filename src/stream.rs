use consumer::*;

pub trait Stream<T> {
    fn consume<C>(self, C) where C: Consumer<T>;
}

pub trait StreamRef<T> {
    fn consume<C>(self, C) where C: ConsumerRef<T>;
}

pub trait StreamRefMut<T> {
    fn consume<C>(self, C) where C: ConsumerRefMut<T>;
}