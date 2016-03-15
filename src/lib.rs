use std::default::Default;
use std::ops;
use std::rc::Rc;
use std::cell::Cell;
use std::marker::PhantomData;


mod consumer;
mod filter;
mod fold;
mod iter;
mod map;
mod producer;
mod stream;
mod subscription;
mod tap;
mod value;

use consumer::*;
use filter::*;
use fold::*;
use iter::*;
use map::*;
use producer::*;
use stream::*;
use subscription::*;
use tap::*;
use value::*;

#[cfg(test)]
mod tests {
    use super::*;
    use consumer::*;
    use filter::*;
    use fold::*;
    use iter::*;
    use map::*;
    use producer::*;
    use stream::*;
    use subscription::*;
    use tap::*;

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
