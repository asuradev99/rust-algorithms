use std::f64::consts::PI;
use num::complex::Complex;

type C64 = Complex<f64>;

#[derive(Debug, Clone)]
pub struct RootOfUnity {
    pub n: isize,
    pub p: isize,
    k: isize,
}

//Symbolic implementation of a primitive nth root of unity to be used in the Fast Fourier Transform.
impl RootOfUnity {
    pub fn new(n: isize) -> Self {
        RootOfUnity { n: n, p: 1, k: 1 }
    }
    fn eval(&self) -> C64 {
        Complex::exp(Complex::new(
            0.0,
            (self.p * 2 * self.k) as f64 * PI / self.n as f64
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

//Main fast fourier transform subroutine, which uses the Cooley-Tukey algorithm to multiply two large numbers; both input and output are vectors of complex floating-point numbers
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
    let c = (z.n / z.k) as usize;
    let mut P: Vec<C64> = vec![Complex::new(0.0, 0.0); c];
    for i in 0..(c / 2) {
        P[i] = E[i] + z.pow(i as isize).eval() * O[i];
        P[i + c / 2] = E[i] - z.pow(i as isize).eval() * O[i];
    }
    P
}

//Uses the fft for polynomial multiplication, with a complexity of approximately O(nlog(n))
pub fn fft_poly(a: Vec<C64>, b: Vec<C64>) -> Vec<usize> {
    let z = RootOfUnity::new((a.len() * 2) as isize);
    let A = fft(&a, &z);
    let B = fft(&b, &z);
    let C = A.iter().zip(B.iter()).map(|(&x, &y)| x * y).collect();
    let C = fft(&C, &RootOfUnity{n: z.n, p: -1, k: z.k});
    C.iter().map(|&c|(c / Complex::new((a.len() * 2) as f64, 0.0)).re.round() as usize).collect()
}



