#![allow(unused_imports, dead_code, non_snake_case, unused_variables)]

#![feature(test)]
extern crate test;

use test::Bencher; 
use rand::thread_rng;
use rand::seq::SliceRandom;

use num::complex::Complex;

mod sorting;
mod fft;

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

#[bench]
fn fft_test(b: &mut Bencher) {
    b.iter(||{
        let a: Vec<u8> = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        let b: Vec<u8> = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];

        let z: fft::RootOfUnity = fft::RootOfUnity::new(a.len() * 2); 

        let A = fft::fft(&a, &z);
        let B = fft::fft(&b, &z);

        let C: Vec<Complex<f64>> = A.iter().zip(B.iter()).map(|(&x, &y)| x * y).collect();
    })
}
