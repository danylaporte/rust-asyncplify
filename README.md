[![Build Status](https://travis-ci.org/danylaporte/rust-asyncplify.svg?branch=master)](https://travis-ci.org/danylaporte/rust-asyncplify)

# asyncplify
Functional Reactive Programming library (RX like) for [Rust](https://github.com/rust-lang/rust)

## Roadmap
This can almost be considered an alpha version.

I am working on integrating more operators and adaptors such as:

- Sorting
- Merging, combine, combine, join (only zip done for now)
- Subjects, sharing of stream
- Schedulers, multi-threading
- Retrying and abort streaming


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
    (0..10)
        .to_stream()        // convert the iterator into a to_stream
        .map(|v| v + 10)    // add 10 to all elements of the stream
        .sum()              // sum it up
        .inspect(|v| println!("the sum is {}", *v)) // print the sum
        .subscribe();       // subscribe to the stream, similar to collect
}
```

##License
The MIT License (MIT)
Copyright (c) 2016 Dany Laporte