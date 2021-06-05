#![allow(dead_code, non_snake_case, unused_variables)]

mod sorting;
mod fft;

use num::complex::Complex;

fn main() {
    let a: Vec<u8> = vec![1, 9, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    let b: Vec<u8> = vec![1, 1, 1, 1, 1, 9, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];

    let z: fft::RootOfUnity = fft::RootOfUnity::new(a.len() * 2); 

    let A = fft::fft(&a, &z);
    let B = fft::fft(&b, &z);

    let C: Vec<Complex<f64>> = A.iter().zip(B.iter()).map(|(&x, &y)| x * y).collect();

    println!("{:?}", C)
}
