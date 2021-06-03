mod sorting;
mod fft;

fn main() {
    let A: Vec<u8> = vec![1, 2, 3, 4];
    let B: Vec<u8> = vec![1, 2, 3, 4];

    let z: fft::RootOfUnity = fft::RootOfUnity::new((A.len() * 2) as u64); 

    println!("{:?}", fft::fft(A, z));
}
