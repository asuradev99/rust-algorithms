#![allow(unused_imports, dead_code, non_snake_case, unused_variables)]
#![feature(test)]
extern crate test;

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use test::Bencher;

use num::complex::Complex;
pub mod mul;
pub mod fft;
pub mod sorting;

// #[bench]
// fn merge_test(b: &mut Bencher) {

//     b.iter(|| {
//         let min: i32 = 0;
//         let max: i32 = 10000;
//         let mut v: Vec<i32> = (min..max).collect();
//         v.shuffle(&mut thread_rng());
//         sorting::merge_sort(&mut v[..]);
//     });
// }

// #[bench]
// fn insertion_test(b: &mut Bencher) {
//     b.iter(|| {
//         let min: i32 = 0;
//         let max: i32 = 10000;
//         let mut v: Vec<i32> = (min..max).collect();
//         v.shuffle(&mut thread_rng());
//         sorting::insertion_sort(&mut v[..]);
//     });
// }

// #[bench]
// fn selection_test(b: &mut Bencher) {
//     b.iter(|| {
//         let min: i32 = 0;
//         let max: i32 = 10000;
//         let mut v: Vec<i32> = (min..max).collect();
//         v.shuffle(&mut thread_rng());
//         sorting::selection_sort(&mut v[..]);
//     });
// }

// #[bench]
// fn fft_test(b: &mut Bencher) {
//     b.iter(|| {
//         let mut rng = rand::thread_rng();
//         let n: i64 = 65536;
//         let a: Vec<u8> = (0..n).map(|_| rng.gen_range(0..10)).collect();
//         let b: Vec<u8> = (0..n).map(|_| rng.gen_range(0..10)).collect();

        
//     })
// }
// #[bench]
// fn mul_test(b: &mut Bencher) {
//     b.iter(|| {
//         let mut rng = rand::thread_rng();
//         let n: i64 = 65536;
//         let a: Vec<u8> = (0..n).map(|_| rng.gen_range(0..10)).collect();
//         let b: Vec<u8> = (0..n).map(|_| rng.gen_range(0..10)).collect();

//         println!("{:?} * {:?} (standard)", a, b);

//         mul::mult(a, b);
//     })
// }
