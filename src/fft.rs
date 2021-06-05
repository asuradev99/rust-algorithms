#![allow(dead_code, non_snake_case, unused_variables)]

use std::f64::consts::PI;

use num::complex::Complex;


type C64 = Complex<f64>;

#[derive(Debug, Clone)]
pub struct RootOfUnity {
    pub n: usize,
    k: usize,
}

impl RootOfUnity {
    pub fn new(n: usize) -> Self {
        RootOfUnity {
            n: n,
            k: 1,
        }
    }
    fn eval(&self) -> C64{
        // PRECOMPUTE e^(2pi) in future refactoring
        Complex::exp(Complex::new(0.0, 2 as f64 * self.k as f64 * PI / self.n as f64))
    }
    fn pow(&self, j: usize) -> RootOfUnity {
        RootOfUnity {
            n: self.n,
            k: self.k * j, 
        }
    }
}

pub fn fft(p: &Vec<u8>,  z: &RootOfUnity) -> Vec<C64> {
    if z.n == z.k {
         return vec![Complex::from(p.iter().sum::<u8>() as f64)]
    } 
    let mut a: [Vec<u8>; 2] = [Vec::new(), Vec::new()];
    for i in 0..(p.len()) {
        a[i % 2].push(p[i]);
    }
    let E = fft(&a[0], &z.pow(2));
    let O = fft(&a[1], &z.pow(2));
    let C = z.n / z.k; 
    let mut P: Vec<C64> = vec![Complex::new(0.0, 0.0); C];
    for i in 0..(C / 2) {
        P[i] = E[i] + z.pow(i).eval() * O[i]; 
        P[(i + (C / 2))] = E[i] - z.pow(i).eval() * O[i]; 
    }
    return P
}


pub fn ifft(p: &Vec<C64>,  z: &RootOfUnity) -> Vec<C64> {
    if z.n == z.k {
        return vec![p.iter().sum()];
    } 
    let mut a: [Vec<C64>; 2] = [Vec::new(), Vec::new()];
    for i in 0..(p.len()) {
        a[i % 2].push(p[i]);
    }
    let E = ifft(&a[0], &z.pow(2));
    let O = ifft(&a[1], &z.pow(2));
    let C = z.n / z.k; 
    let mut P: Vec<C64> = vec![Complex::new(0.0, 0.0); C];
    for i in 0..(C / 2) {
        P[i] = E[i] + z.pow(i).eval() * O[i]; 
        P[(i + (C / 2))] = E[i] - z.pow(i).eval() * O[i]; 
    }
    return P
}