#![allow(dead_code, non_snake_case, unused_variables)]

extern crate algorithms; 

pub mod fft; 
pub mod mul; 

use num::complex::Complex;

type C64 = Complex<f64>;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let n: i64 = 4;
    let a: Vec<C64> = (0..n).map(|_| Complex::from(rng.gen_range(0..10) as f64)).collect();
    let b: Vec<C64> = (0..n).map(|_| Complex::from(rng.gen_range(0..10) as f64)).collect();

    println!("{:?} + {:?}", a, b);

    println!(" result: {:?}", mul::mult(vec![1, 2, 3, 4], vec![5, 6, 7, 8]));
}
