#![cfg(test)]

use test::{Bencher,black_box};
use asyncplify::*;
use std::iter;

#[bench]
fn iter_count(b: &mut Bencher) {
    b.iter(|| (0..black_box(1000)).count());
}

#[bench]
fn stream_count(b: &mut Bencher) {
    b.iter(|| (0..black_box(1000)).to_stream().count().last_value());
}

#[bench]
fn iter_map(b: &mut Bencher) {
    b.iter(|| (0..black_box(1000)).map(|i| i + 2).last());
}

#[bench]
fn stream_map(b: &mut Bencher) {
    b.iter(|| (0..black_box(1000)).to_stream().map(|i| i + 2).last_value());
}

#[bench]
fn iter_max(b: &mut Bencher) {
    b.iter(|| (0..black_box(1000)).max());
}

#[bench]
fn stream_max(b: &mut Bencher) {
    b.iter(|| (0..black_box(1000)).to_stream().max().last_value());
}

#[bench]
fn iter_max_by_key(b: &mut Bencher) {
    b.iter(|| (0..black_box(1000)).max_by_key(|v| v + 1));
}

#[bench]
fn stream_max_by_key(b: &mut Bencher) {
    b.iter(|| (0..black_box(1000)).to_stream().max_by_key(|v| v + 1).last_value());
}

#[bench]
fn iter_flat_map(b: &mut Bencher) {
    b.iter(|| (0..black_box(1000)).flat_map(|i| iter::once(i)).last());
}

#[bench]
fn stream_flat_map(b: &mut Bencher) {
    b.iter(|| (0..black_box(1000)).to_stream().flat_map(|i| once(i)).last_value());
}