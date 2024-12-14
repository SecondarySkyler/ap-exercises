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

// implement this for i32 and String
// i32 -> divide by 2
// String -> return the first half
trait Half {
    fn half(&self) -> Self;
}

// this fn should return something but I dont recall (probably T)
fn func<T: MakePowerFull + Half>(el: T) -> T {
    todo!()
}

// implement this for i32 and String
trait Appendable {
    fn append(&self, original: &mut String) -> String;
}

fn get_vec() -> Vec<Box<dyn Appendable>> {
    Vec::new()
}

// The returned value should be the result of the append of all the items in v
fn func1(v: &Vec<Box<dyn Appendable>>) -> String {
    todo!()
}

// Given a range, the iterator should advance by step quantity
struct Interval {
    range: Range<u32>,
    step: u32
}

impl Iterator for Interval {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}