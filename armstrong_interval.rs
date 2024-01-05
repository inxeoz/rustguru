use std::cmp::{max, min};
use crate::armstrong::arm;

pub fn a2b(a: usize, b: usize){
    let a = min(a, b);
    let b = max(a, b);
    for i in a..b {
        match arm(i) {
            (true, n) => println!(" {n}"),
            (false, n) => {},

        }
    }
}