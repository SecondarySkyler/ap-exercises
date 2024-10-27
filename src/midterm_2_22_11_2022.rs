#![allow(unused)]
use std::fmt::Debug;

// Exercise 1
trait Doublable {
    fn gimme_double(&self) -> Self;
}

impl Doublable for i32 {
    fn gimme_double(&self) -> Self {
       self * 2 
    }
}

impl Doublable for String {
    fn gimme_double(&self) -> Self {
        self.repeat(2)
    }
}

fn printdouble<T: Doublable + Debug>(input: T) {
    println!("doubling {:?} is {:?}", input, input.gimme_double());
}

// Exercise 2
struct Wrapper {
    v: Vec<i32>
}

#[cfg(test)]
mod mt_2_2022 {
    use super::*;

    #[test]
    fn test_ex1() {
        let x = 5;
        assert_eq!(10, x.gimme_double());
        let s = "what".to_string();
        assert_eq!("whatwhat".to_string(), s.gimme_double());
    }
}
