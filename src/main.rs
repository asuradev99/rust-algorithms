#![allow(dead_code, non_snake_case, unused_variables)]

mod sorting;
mod fft;

use num::complex::Complex;

//absolutely horrible function here, just made it as quick as possible, will fix soon
fn main() {
    let a: Vec<u8> = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    let b: Vec<u8> = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];

    let mut z: fft::RootOfUnity = fft::RootOfUnity::new((a.len() * 2) as isize); 

    let A = fft::fft(&a, &z);
    let B = fft::fft(&b, &z);

    let C: Vec<Complex<f64>> = A.iter().zip(B.iter()).map(|(&x, &y)| x * y).collect();
    z.p = -1; 

    let D: Vec<Complex<f64>> = fft::ifft(&C, &z);
    let mut E: Vec<f64> = Vec::new();

    for i in D {
        let e: Complex<f64> = i / Complex::new((a.len() * 2) as f64, 0.0);
        E.push(e.re.round());
    }

    println!("{:?}", E)

}
