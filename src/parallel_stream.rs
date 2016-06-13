trait ParallelStream<T: Send> {
    fn consume<C: Consumer<T> + Send>(C);
}