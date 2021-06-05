#![allow(dead_code, non_snake_case, unused_variables)]

use std::f64::consts::PI;

use num::complex::Complex;


type C64 = Complex<f64>;

#[derive(Debug, Clone)]
pub struct RootOfUnity {
    pub n: u64,
    k: u64
}

impl RootOfUnity {
    pub fn new(n: u64) -> Self {
        RootOfUnity {
            n: n,
            k: 1
        }
    }
    fn eval(&self) -> C64{
        // PRECOMPUTE e^(2pi) in future refactoring
        Complex::exp(Complex::new(0.0, 2 as f64 * self.k as f64 * PI / self.n as f64))
    }
    fn pow(&self, j: u64) -> RootOfUnity {
        RootOfUnity {
            n: self.n,
            k: self.k * j
        }
    }
}

pub fn fft (p: Vec<u8>,  z: &RootOfUnity) -> Vec<C64>{

    if z.n == z.k {
        return vec![Complex::new((p.iter().sum::<u8>()) as f64, 0.0 )];
    } 

    let (mut e, mut o): (Vec<u8>, Vec<u8>) = (Vec::new(), Vec::new());

    for i in 0..(p.len()) {
        if i % 2 == 0 {
            e.push(p[i]);
        }
        else {
            o.push(p[i]);
        }
    }

    let E = fft(e, &z.pow(2));
    let O = fft(o, &z.pow(2));

    let C = z.n / z.k; 
    let mut P: Vec<C64> = vec![Complex::new(0.0, 0.0); C as usize];
    for i in 0..(C / 2) {
        P[i as usize] = E[i as usize] + z.pow(i).eval() * O[i as usize]; 
        P[(i + (C / 2)) as usize] = E[i as usize] - z.pow(i).eval() * O[i as usize]; 
    }

    return P
}

