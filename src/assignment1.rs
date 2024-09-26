#![allow(dead_code)]
use std::collections::HashMap;
pub fn string_reverse(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn bigger(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

pub fn multiply(a: i32, b: f32, c: f64) -> f64 {
    (a as f64) * (b as f64) * c
}

const C: f32 = 299_792_458.0; 
pub fn energy(m: f32) -> f32 {
    m * C.powi(2)
}

pub fn max_min(v: &Vec<i32>) -> (i32, i32) {
    let mut max = v[0];
    let mut min = v[0];
    for i in v {
        if *i > max {
            max = *i;
        }
        if *i < min {
            min = *i;
        }
    }
    (max, min)
}

pub fn lord_farquad(s: String) -> String {
    s.replace('e', "BOOM")
}

pub fn get_price(furniture: &HashMap<String, f32>, item: String) -> f32 {
    match furniture.get(&item) {
        Some(&price) => price,
        None => -1.0,
    }
}

pub fn append(s: String) -> String {
    s + "foobar"
}

pub fn is_armstrong(n: i32) -> bool {
    let mut sum = 0;
    let mut temp = n;
    let mut digits = Vec::new();
    while temp > 0 {
        digits.push(temp % 10);
        temp /= 10;
    }
    let len = digits.len() as i32;
    for i in digits {
        sum += i.pow(len as u32);
    }
    sum == n
}

pub type Matrix = ((i32, i32), (i32, i32));

pub fn transpose(m: Matrix) -> Matrix {
    ((m.0.0, m.1.0), (m.0.1, m.1.1))
}

#[cfg(test)]
mod test1 {
    use super::*;

    #[test]
    fn test_bigger() {
        assert_eq!(bigger(10, 20), 20);
        assert_eq!(bigger(20, 10), 20);
    }
}