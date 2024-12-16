#![allow(unused)]

// MasterClock and SlaveClock
// List<T> with add, append, prepend, get

use std::{ops::Range};

// implement this for i32 and String
// i32 -> pow
// String -> to uppercase
trait MakePowerFull {
    fn makepowerfull(&self) -> Self;
}

impl MakePowerFull for i32 {
    fn makepowerfull(&self) -> Self {
        return self.pow(2);
    }
}

impl MakePowerFull for String {
    fn makepowerfull(&self) -> Self {
        return self.to_uppercase();
    }
}

// implement this for i32 and String
// i32 -> divide by 2
// String -> return the first half
trait Half {
    fn half(&self) -> Self;
}

impl Half for i32 {
    fn half(&self) -> Self {
        return self / 2
    }
}

impl Half for String {
    fn half(&self) -> Self {
        let mut cl = self.clone();
        let tmp = cl.split_off(self.len() / 2);
        return cl;
    }
}

// this fn should return something but I dont recall (probably T)
// Unfortunately I dont remember what this function should do
fn func<T: MakePowerFull + Half>(el: T) -> T {
    todo!()
}

// implement this for i32 and String
trait Appendable {
    fn append(&self, original: &mut String) -> String;
}

impl Appendable for i32 {
    fn append(&self, original: &mut String) -> String {
        original.push_str(self.to_string().as_str());
        return original.to_string()
    }
}

impl Appendable for String {
    fn append(&self, original: &mut String) -> String {
        original.push_str(self.as_str());
        return original.to_string()
    }
}

fn get_vec() -> Vec<Box<dyn Appendable>> {
    Vec::new()
}

// The returned value should be the result of the append of all the items in v
fn func1(v: &Vec<Box<dyn Appendable>>) -> String {
    let mut result = String::new();

    for el in v {
        result = el.append(&mut result);
    }

    return result;
}

// Given a range, the iterator should advance by step quantity
struct Interval {
    pub range: Range<u32>,
    pub step: u32
}

impl Iterator for Interval {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.range.is_empty() {
            let current = self.range.start;
            self.range.start += self.step;
            return Some(current);
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod mt_2_18_11_2024 {
    use super::*;

    #[test]
    fn test_powerfull_half() {
        let n: i32 = 10;
        let s: String = String::from("Hello");

        assert_eq!(n.makepowerfull(), 100);
        assert_eq!(s.makepowerfull(), String::from("HELLO"));

        assert_eq!(n.half(), 5);
        assert_eq!(s.half(), String::from("He"));
    }

    #[test]
    fn test_appendable() {
        let vec: Vec<Box<dyn Appendable>> = vec![Box::new(10), Box::new(20), Box::new("ciao".to_string()), Box::new(30), Box::new("pizza".to_string())];
        assert_eq!(func1(&vec), "1020ciao30pizza");
    }

    #[test]
    fn test_range() {
        let inter = Interval { range: (0..100), step: 10};
        for i in inter {
            println!("{i}");
        }
    }
}