mod consumer;
mod count;
mod empty;
mod filter;
mod flatmap;
mod fold;
mod group_by;
mod into_vec;
mod iter;
mod map;
mod max;
mod max_by;
mod min;
mod min_by;
mod producer;
mod stream;
mod subscription;
mod sum;
mod take;
mod take_last;
mod tap;
mod to_vec;
mod value;

pub use consumer::*;
pub use count::*;
pub use empty::*;
pub use filter::*;
pub use flatmap::*;
pub use fold::*;
pub use group_by::*;
pub use into_vec::*;
pub use iter::*;
pub use map::*;
pub use max::*;
pub use max_by::*;
pub use min::*;
pub use min_by::*;
pub use producer::*;
pub use stream::*;
pub use subscription::*;
pub use sum::*;
pub use take::*;
pub use take_last::*;
pub use tap::*;
pub use to_vec::*;
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
