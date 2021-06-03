#![allow(dead_code)]

use std::fmt;
pub fn insertion_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] >  arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}
 
pub fn selection_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 0..(arr.len() - 1) {
        let mut s = i;
        for j in (i + 1)..arr.len() {
            if arr[j] < arr[s] {
                s = j;
            }
        }
        arr.swap(i, s);
    }
}

pub fn merge_sort<T: PartialOrd + Copy + fmt::Debug>(arr: &mut [T]) { 
    if arr.len() > 1 {
        let m: usize = arr.len() / 2;

        merge_sort::<T>(&mut arr[0..m]);
        merge_sort::<T>(&mut arr[m..]);

        let v: Vec<T> = arr.to_vec();
        merge(arr, &v[0..m], &v[m..]);
    }
}

pub fn merge<T: PartialOrd + Copy + fmt::Debug>(arr: &mut [T], l: &[T], r: &[T]) {
    let (mut i, mut j, mut k) = (0, 0, 0);
    while i < l.len() && j < r.len() {
        if l[i] <= r[j] {
            arr[k] = l[i];
            i = i + 1;
        } else if l[i] >= r[j] {
            arr[k] = r[j];
            j = j + 1;
        }
        k = k + 1;
    }
    if i == l.len() {
        arr[k..].copy_from_slice(&r[j..]);
    } else {
        arr[k..].copy_from_slice(&l[i..]);
    }
   
}