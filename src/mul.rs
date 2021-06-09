#![allow(dead_code, non_snake_case, unused_variables)]
use std::cmp::Ordering;

//subroutine to multiply a vector of digits by a single digit. Due to the potentially huge amount of digits in the input, numbers are handled as a series 
//of digits. 
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

//Subroutine to add two vectors of digits to each other. 
pub fn add(A: &mut Vec<u8>, B: &mut Vec<u8>) -> Vec<u8> {
    match A.len().cmp(&B.len()) {
        Ordering::Less => A.resize(B.len(), 0),
        Ordering::Greater => B.resize(A.len(), 0),
        Ordering::Equal => {},
    }
    let mut s = A.iter().zip(B.iter()).map(|(&a, &b)| a + b).collect::<Vec<u8>>();
    let mut i = 0;
    while s[i] > 9 {
        if i == s.len() {
            s.push(0);
        }
        s[i + 1] = s[i + 1] + s[i] / 10;
        s[i] = s[i] % 10;
        i = i + 1; 
    } 
    s
}

//Driver function to multiply two vectors of digits together using the previous two functions. 
pub fn mult (A: Vec<u8>, B: Vec<u8>) -> Vec<u8>{
    let mut P: Vec<u8> = Vec::new();
    for i in 0..B.len() {
        let mut r = vec![0; i]; 
        r.append(&mut muls(&A, B[i]));
        P = add(&mut P, &mut r);
    }
    P
}