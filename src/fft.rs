#![allow(dead_code, non_snake_case, unused_variables)]

use std::f64::consts::PI;

use num::complex::Complex;

type C64 = Complex<f64>;

#[derive(Debug, Clone)]
pub struct RootOfUnity {
    pub n: isize,
    pub p: isize,
    k: isize,
}

impl RootOfUnity {
    pub fn new(n: isize) -> Self {
        RootOfUnity { n: n, p: 1, k: 1 }
    }
    fn eval(&self) -> C64 {
        // PRECOMPUTE e^(2pi) in future refactoring
        Complex::exp(Complex::new(
            0.0,
            self.p as f64 * 2 as f64 * self.k as f64 * PI / self.n as f64,
        ))
    }
    fn pow(&self, j: isize) -> RootOfUnity {
        RootOfUnity {
            n: self.n,
            p: self.p,
            k: self.k * j,
        }
    }
}

pub fn fft(p: &Vec<C64>, z: &RootOfUnity) -> Vec<C64> {
    if z.n == z.k {
        return vec![p.iter().sum()];
    }
    let mut a: [Vec<C64>; 2] = [Vec::new(), Vec::new()];
    for i in 0..(p.len()) {
        a[i % 2].push(p[i]);
    }
    let E = fft(&a[0], &z.pow(2));
    let O = fft(&a[1], &z.pow(2));
    let C = (z.n / z.k) as usize;
    let mut P: Vec<C64> = vec![Complex::new(0.0, 0.0); C];
    for i in 0..(C / 2) {
        P[i as usize] = E[i as usize] + z.pow(i as isize).eval() * O[i as usize];
        P[(i as usize + (C / 2))] = E[i] - z.pow(i as isize).eval() * O[i];
    }
    return P;
}

pub fn fft_poly(a: Vec<C64>, b: Vec<C64>) -> Vec<usize> {
    let mut z: RootOfUnity = RootOfUnity::new((a.len() * 2) as isize);
    let A = fft(&a, &z);
    let B = fft(&b, &z);
    let C: Vec<Complex<f64>> = A.iter().zip(B.iter()).map(|(&x, &y)| x * y).collect();
    z.p = -1;
    let D: Vec<Complex<f64>> = fft(&C, &z);
    let mut E: Vec<usize> = Vec::new();
    for i in D {
        let e: Complex<f64> = i / Complex::new((a.len() * 2) as f64, 0.0);
        E.push(e.re.round() as usize);
    }
    E
}
