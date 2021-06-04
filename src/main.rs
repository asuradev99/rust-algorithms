mod sorting;
mod fft;

use num::complex::Complex;

fn main() {
    let a: Vec<u8> = vec![1, 2, 3, 4];
    let b: Vec<u8> = vec![1, 2, 3, 4];

    let z: fft::RootOfUnity = fft::RootOfUnity::new((a.len() * 2) as u64); 

    let A = fft::fft(a, &z);
    let B = fft::fft(b, &z);

    let C: Vec<Complex<f64>> = A.iter().zip(B.iter()).map(|(&x, &y)| x * y).collect();

    println!("{:?}", C)
}
