#![allow(unused)]
use std::{fmt::Debug, usize};

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
    pub v: Vec<i32>
}

struct WrapperIterator<'a> {
    container: &'a Wrapper,
    index: usize
}

impl Wrapper {
    fn iter(&self) -> WrapperIterator {
        WrapperIterator {
            container: self,
            index: 1
        }
    }
}

impl<'a> Iterator for WrapperIterator<'a> {
    type Item = i32;

   fn next(&mut self) -> Option<Self::Item> {
       if self.index < self.container.v.len() {
           let num = Some(self.container.v[self.index]);
           self.index += 2;
           num
       } else {
           None
       }
   } 
}

// Exercise 3
fn basicbox_sum(v: Vec<String>) -> Vec<Box<usize>> {
    let mut total_sum: usize = 0;
    let mut vb: Vec<Box<usize>> = Vec::new();

    for string in v {
        let length = string.len();
        total_sum += length;
        vb.push(Box::new(length));
    }
    vb.push(Box::new(total_sum));
    vb
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

    #[test]
    fn test_ex2() {
        let w = Wrapper { v: vec![1,2,3,4,5,6,7,8] };
        let mut iw = w.iter();
        assert_eq!(iw.next().unwrap() , 2);
        assert_eq!(iw.next().unwrap() , 4);
        assert_eq!(iw.next().unwrap() , 6);
        assert_eq!(iw.next().unwrap() , 8);
    }

    #[test]
    fn test_ex3() {
        let s = vec!["asd".to_string(), "where".to_string(), "what".to_string()];
        let sb = basicbox_sum(s);
        let sb3: Vec<Box<usize>> = vec![Box::new(3), Box::new(5), Box::new(4), Box::new(12)];
        assert_eq!(sb, sb3); 
    }
}
