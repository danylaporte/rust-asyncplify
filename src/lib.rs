mod consumer;
mod count;
mod empty;
mod filter;
mod flatmap;
mod fold;
mod group_by;
mod inspect;
mod into_vec;
mod iter;
mod map;
mod max_by_key;
mod max;
mod min_by_key;
mod min;
mod skip_last;
mod skip;
mod stream;
mod subscription;
mod sum;
mod take_last;
mod take;
mod to_vec;
mod value;

pub use consumer::*;
pub use count::*;
pub use empty::*;
pub use filter::*;
pub use flatmap::*;
pub use fold::*;
pub use group_by::*;
pub use inspect::*;
pub use into_vec::*;
pub use iter::*;
pub use map::*;
pub use max_by_key::*;
pub use max::*;
pub use min_by_key::*;
pub use min::*;
pub use skip_last::*;
pub use skip::*;
pub use stream::*;
pub use subscription::*;
pub use sum::*;
pub use take_last::*;
pub use take::*;
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
            .inspect(|v| f = *v)
            .subscribe();

        assert!(f == 4950, "f = {}", f);
    }
}
