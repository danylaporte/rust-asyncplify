mod clonable;
mod consumer;
mod count;
mod dedup_by_key;
mod dedup;
mod empty;
mod filter;
mod flat_map;
mod fold;
mod group_by;
mod inspect;
mod interval;
mod into_vec;
mod iter;
mod last_value;
mod map;
mod max_by_key;
mod max;
mod min_by_key;
mod min;
mod once;
mod scan;
mod skip_last;
mod skip_until;
mod skip;
mod sort;
mod stream;
mod subscription;
mod sum;
mod take_last;
mod take_until;
mod take;
mod to_vec;
mod unique_by_key;
mod unique;
mod zip;

pub use clonable::*;
pub use consumer::*;
pub use count::*;
pub use dedup_by_key::*;
pub use dedup::*;
pub use empty::*;
pub use filter::*;
pub use flat_map::*;
pub use fold::*;
pub use group_by::*;
pub use inspect::*;
pub use interval::*;
pub use into_vec::*;
pub use iter::*;
pub use map::*;
pub use max_by_key::*;
pub use max::*;
pub use min_by_key::*;
pub use min::*;
pub use once::*;
pub use scan::*;
pub use skip_last::*;
pub use skip_until::*;
pub use skip::*;
pub use sort::*;
pub use stream::*;
pub use subscription::*;
pub use sum::*;
pub use take_last::*;
pub use take_until::*;
pub use take::*;
pub use to_vec::*;
pub use unique_by_key::*;
pub use unique::*;
pub use zip::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut f = 0i16;

        (0..100)
            .to_stream()
            .fold(0i16, |v, i| v + i)
            .subscribe_action(|v| f = v);
        assert!(f == 4950, "f = {}", f);
    }
}
