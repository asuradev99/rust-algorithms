#![allow(dead_code, non_snake_case, unused_variables)]

mod fft;
mod sorting;
mod mul; 

use num::complex::Complex;
use rand::Rng;

//absolutely horrible function here, just made it as quick as possible, will fix soon
fn main() {
    
    //println!("{:?}", mul::mult(A, B));

    let mut rng = rand::thread_rng();
    let n: i64 = 4;
    let a: Vec<u8> = (0..n).map(|_| rng.gen_range(0..10)).collect();
    let b: Vec<u8> = (0..n).map(|_| rng.gen_range(0..10)).collect();

    println!("{:?} * {:?}", a, b);

    println!("FFT result: {:?}", fft::fft_poly(a.clone(), b.clone()));
    println!("Mul result: {:?}", mul::mult(a, b));
}
