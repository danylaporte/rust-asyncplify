pub trait Consumer<T> {
    fn emit(&mut self, T) -> bool;
}