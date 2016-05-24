use consumer::*;

// A container containing the last value
pub struct LastValue<T> {
    pub value: Option<T>,
}

impl<'a, T> Consumer<T> for &'a mut LastValue<T> {
    fn emit(&mut self, item: T) -> bool {
        self.value = Some(item);
        true
    }
}

impl<T> LastValue<T> {
    pub fn new() -> Self {
        LastValue { value: None }
    }
}