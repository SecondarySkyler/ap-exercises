#![allow(dead_code)]
use std::collections::HashMap;
use std::slice::Iter;

pub fn modify_odd(slice: &mut [i32]) {
    for i in 0..slice.len() {
        if slice[i] % 2 != 0 {
            slice[i] = 0;
        }
    }
}

pub fn count_characters(s: String) -> HashMap<char, u32> {
    let mut map = HashMap::new();
    for c in s.chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }
    map
}

pub fn split_at_value(slice: &[i32], value: i32) -> Option<(&[i32], &[i32])> {
    for i in 0..slice.len() {
        if slice[i] == value {
            return Some((&slice[0..i], &slice[i+1..]));
        }
    }
    None
}

// Do the order count?
pub fn sub_slice(v1: &Vec<i32>, v2: &Vec<i32>) {
    if v2.len() > v1.len() {
        println!("Not found");
        return;
    }
}

// Start of Ex. 5
pub fn max(v: &Vec<i32>) -> Option<i32> {
    if let Some(max) = v.iter().max() {
        Some(*max)
    } else {
        None
    }
}

pub fn swap(v: &mut Vec<i32>) {
    if v.len() <= 1 {
        return;
    }

    let last_index = v.len() - 1;
    v.swap(0, last_index);
}

pub fn is_sorted(v: &Vec<i32>) -> bool {
    for i in 1..v.len() {
        if v[i] < v[i - 1] {
            return false;
        }
    }
    true
}

pub fn insert_if_longer(v: &mut Vec<String>, s: String) {
    if s.len() >= 10 {
        v.push(s);
    }
}
// End of Ex. 5

pub fn build_vector(i: Iter<i32>) -> Vec<&i32> {
    i.collect()
}

pub fn pancake_sort(v: &mut Vec<i32>) {
    for i in (1..v.len()).rev() {
        let mut max_index = 0;
        for j in 1..=i {
            if v[j] > v[max_index] {
                max_index = j;
            }
        }
        v[0..=max_index].reverse();
        v[0..=i].reverse();
    }
}

pub fn merge(x: &[i32], y: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < x.len() && j < y.len() {
        if x[i] < y[j] {
            result.push(x[i]);
            i += 1;
        } else {
            result.push(y[j]);
            j += 1;
        }
    }
    result.extend_from_slice(&y[j..]);
    result.extend_from_slice(&x[i..]);
    result
}

pub enum DoubleType {
    Int(i32),
    Str(String)
}

pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide
}

pub enum Expression {
    Number(i32),
    Operation {
        left: Box<Expression>, 
        op:Operation,
        right: Box<Expression>
    }
}

pub fn evaluate_expression(exp: &Expression) -> Result<i32, &str> {
    match exp {
        Expression::Number(n) => Result::Ok(*n),
        Expression::Operation {left, op, right} => {
            let left_value = evaluate_expression(left)?;
            let right_value = evaluate_expression(right)?;

            match op {
                Operation::Add => {
                    let result = left_value.checked_add(right_value);
                    match result {
                        Option::None => Result::Err("error"),
                        Option::Some(r) => Result::Ok(r)
                    }
                }
                Operation::Subtract => {
                    let result = left_value.checked_sub(right_value);
                    match result {
                        Option::None => Result::Err("error"),
                        Option::Some(r) => Result::Ok(r)
                    }
                }
                Operation::Multiply => {
                    let result = left_value.checked_mul(right_value);
                    match result {
                        Option::None => Result::Err("error"),
                        Option::Some(r) => Result::Ok(r)
                    }
                }
                Operation::Divide => {
                    let result = left_value.checked_div(right_value);
                    match result {
                        Option::None => Result::Err("error"),
                        Option::Some(r) => Result::Ok(r)
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test2 {
    use super::*;

    #[test]
    fn test_count_characters() {
        let s = "hello".to_string();
        let map = count_characters(s);
        assert_eq!(map.get(&'h'), Some(&1));
        assert_eq!(map.get(&'e'), Some(&1));
        assert_eq!(map.get(&'l'), Some(&2));
        assert_eq!(map.get(&'o'), Some(&1));
    }
}