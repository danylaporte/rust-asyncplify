[![Build Status](https://travis-ci.org/danylaporte/rust-asyncplify.svg?branch=master)](https://travis-ci.org/danylaporte/rust-asyncplify)

# asyncplify
Functional Reactive Programming library (RX like) for [Rust](https://github.com/rust-lang/rust) - [Reference](http://danylaporte.github.io/rust-asyncplify)

## Roadmap
This can almost be considered an alpha version.

I am working on integrating more operators and adaptors such as:

- [ ] Sorting
- [ ] Merging
- [ ] Combining
- [ ] Joining
- [ ] Subjects
- [x] Sharing (Clonable)
- [x] Interval
- [ ] Multi-Thread
- [ ] Retrying
- [x] TakeUntil
- [x] SkipUntil

## Documentation
[Reference](http://danylaporte.github.io/rust-asyncplify)

[RxMarbles](http://rxmarbles.com/)

[Rx Operators](http://reactivex.io/documentation/operators.html)


## Usage

This example create a stream from an iterator, add 10 to all element and sum them up before printing the result.
```rust
extern crate asyncplify;

use asyncplify::*;

fn main() {
    let v = (0..10)
        .to_stream()        // convert the iterator into a stream
        .map(|v| v + 10)    // add 10 to all elements of the stream
        .sum()              // sum it up
        .last_value()       // return the last value
        .unwrap();
        
    println!("the sum is {}", v);
}
```

## License
The MIT License (MIT)
Copyright (c) 2016 Dany Laporte