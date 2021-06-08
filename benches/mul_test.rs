use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

use algorithms::{fft, mul};
use rand::Rng;


use num::complex::Complex;

type C64 = Complex<f64>;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci");
    for i in 1..16 {
        let mut rng = rand::thread_rng();
        let n: i64 = 2i64.pow(i);
        let a: Vec<u8> = (0..n).map(|_| rng.gen_range(0..10)).collect();
        let b: Vec<u8> = (0..n).map(|_| rng.gen_range(0..10)).collect();

        group.bench_with_input(BenchmarkId::new("Standard Multiplication", i), &i, |e, i| e.iter(|| mul::mult(a.clone(), b.clone())));
        let a: Vec<C64> = a.iter().map(|&x| Complex::from(x as f64)).collect();
        let b: Vec<C64> = b.iter().map(|&x| Complex::from(x as f64)).collect();
    
        group.bench_with_input(BenchmarkId::new("Fast Fourier Transform", i), &i, |e, i| e.iter(|| fft::fft_poly(a.clone(), b.clone())));
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);