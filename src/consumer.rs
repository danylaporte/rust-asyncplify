pub trait Consumer<T> {
    fn emit(&mut self, T) -> bool;
}

pub trait ParallelConsumer<T>: Send
    where T: Send
{
    fn emit(&self, T) -> bool;
}
