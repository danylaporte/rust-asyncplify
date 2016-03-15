use consumer::*;

pub trait Stream {
    type Item;

    fn consume<C>(self, C) where C: Consumer<Item = Self::Item>;
}
