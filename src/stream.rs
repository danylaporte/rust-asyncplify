use consumer::*;

pub trait Stream<T> {
    fn consume(self, consumer: &mut Consumer<T>);
}

pub trait StreamRef<T> {
    fn consume(self, consumer: &mut ConsumerRef<T>);
}

pub trait StreamRefMut<T> {
    fn consume(self, consumer: &mut ConsumerRefMut<T>);
}