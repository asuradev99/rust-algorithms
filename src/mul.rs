#![allow(dead_code, non_snake_case, unused_variables)]
use std::cmp::Ordering;

pub fn muls(A: &Vec<u8>, B: u8) -> Vec<u8> {
    let mut P: Vec<u8> = Vec::new(); 
    let mut c = 0; 
    for a in A {
        let p = B * a + c; 
        P.push(p % 10);
        if p > 9 {
            c = p / 10;
        } else {
            c = 0;
        }
    } 
    if c > 0 {
        P.push(c);
    }
    P
}

pub fn add(A: &mut Vec<u8>, B: &mut Vec<u8>) -> Vec<u8> {
    match A.len().cmp(&B.len()) {
        Ordering::Less => A.resize(B.len(), 0),
        Ordering::Greater => B.resize(A.len(), 0),
        Ordering::Equal => {},
    }
    let mut s = A.iter().zip(B.iter()).map(|(&a, &b)| a + b).collect::<Vec<u8>>();
    for i in 0..(s.len() - 1) {
        s[i + 1] = s[i + 1] + s[i] / 10;
        s[i] = s[i] % 10;
    }
    if s[s.len() - 1] > 9 {
        let l = s.len();
        s.push(s[l - 1] / 10);
        s[l - 1] = s[l - 1] % 10;
    }
    s 
}

pub fn mult (A: Vec<u8>, B: Vec<u8>) -> Vec<u8>{
    let mut P: Vec<u8> = Vec::new();
    let mut S: usize = 0; 
    for b in B {
        let mut r = muls(&A, b);
        r.reverse();
        r.resize(r.len() + S, 0);
        r.reverse();
        P = add(&mut P, &mut r);
        S = S + 1;
    }
    P
}