[![Build Status](https://travis-ci.org/danylaporte/rust-asyncplify.svg?branch=master)](https://travis-ci.org/danylaporte/rust-asyncplify)

# asyncplify
Functional Reactive Programming library (RX like) for [Rust](https://github.com/rust-lang/rust)

## Not ready for production yet
This is a work in progress and research project right now. Please consider this lib in alpha stage.

## Documentation
[Reference](http://danylaporte.github.io/rust-asyncplify)

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
        .tap(|v| println!("the sum is {}", *v)) // print the sum
        .subscribe();       // subscribe to the stream, similar to collect
}
``` 

##License
The MIT License (MIT)
Copyright (c) 2016 Dany Laporte