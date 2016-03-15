mod consumer;
mod empty;
mod filter;
mod fold;
mod iter;
mod map;
mod producer;
mod stream;
mod subscription;
mod tap;
mod unit;
mod value;

pub use consumer::*;
pub use empty::*;
pub use filter::*;
pub use fold::*;
pub use iter::*;
pub use map::*;
pub use producer::*;
pub use stream::*;
pub use subscription::*;
pub use tap::*;
pub use unit::*;
pub use value::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut f = 0i16;

        (0..100)
            .to_stream()
            .fold(0i16, |v, i| v + i)
            .tap(|v| f = *v)
            .subscribe();

        assert!(f == 4950, "f = {}", f);
    }
}
