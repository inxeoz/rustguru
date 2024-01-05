use std::cmp::{max, min};
use crate::prime::prm;

pub fn intervals(a: usize, b:usize) {
    let m = min(a, b);
    let mx = max(a, b);

    for i in m..mx {
        if prm(i) {
            print!(" {i} ");
        }
    }
}