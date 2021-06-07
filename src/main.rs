#![allow(dead_code, non_snake_case, unused_variables)]

mod fft;
mod sorting;
mod mul; 


use num::complex::Complex;

type C64 = Complex<f64>;
use rand::Rng;

//absolutely horrible function here, just made it as quick as possible, will fix soon
fn main() {
    //println!("{:?}", mul::mult(A, B));

    let mut rng = rand::thread_rng();
    let n: i64 = 4;
    let a: Vec<C64> = (0..n).map(|_| Complex::from(rng.gen_range(0..10) as f64)).collect();
    let b: Vec<C64> = (0..n).map(|_| Complex::from(rng.gen_range(0..10) as f64)).collect();

    println!("{:?} * {:?}", a, b);

    println!("FFT result: {:?}", fft::fft_poly(a.clone(), b.clone()));
}
