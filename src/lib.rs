#![allow(dead_code)]

#![feature(test)]
extern crate test;

use test::Bencher; 
use rand::thread_rng;
use rand::seq::SliceRandom;

mod sorting;
mod fft;

#[bench]
fn merge_test(b: &mut Bencher) {
    
    b.iter(|| {
        let min: i32 = 0;
        let max: i32 = 10000;
        let mut v: Vec<i32> = (min..max).collect();
        v.shuffle(&mut thread_rng());
        sorting::merge_sort(&mut v[..]);
    });
}

#[bench]
fn insertion_test(b: &mut Bencher) {
    b.iter(|| {
        let min: i32 = 0;
        let max: i32 = 10000;
        let mut v: Vec<i32> = (min..max).collect();
        v.shuffle(&mut thread_rng());
        sorting::insertion_sort(&mut v[..]);
    });
}

#[bench]
fn selection_test(b: &mut Bencher) {
    b.iter(|| {
        let min: i32 = 0;
        let max: i32 = 10000;
        let mut v: Vec<i32> = (min..max).collect();
        v.shuffle(&mut thread_rng());
        sorting::selection_sort(&mut v[..]);
    });
}