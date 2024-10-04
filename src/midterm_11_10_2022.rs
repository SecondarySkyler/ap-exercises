#![allow(dead_code)]

use core::fmt;
use std::path::Display;

use crate::midterm1::prev_char;

// --- QUESTION 9 ---
pub enum XX {
    Y1(i32, i32),
    Y2(String)
}

pub fn data(e: XX) -> i32 {
    match e {
        XX::Y1(n1, n2) => n1 *n2,
        XX::Y2(s) => s.len() as i32
    }
}

// --- QUESTION 10 ---
pub enum Z {
    Y1(i32, i32),
    Y2(bool, String)
}

pub fn maybelen(e: &Z) -> Option<usize> {
    match e {
        Z::Y1(_, _) => None,
        Z::Y2(_, s) => Some(s.len())
    }
}

// --- QUESTION 11 ---
mod enumx {
    pub enum X {
        Y(String)
    }
}

mod structx {
    pub struct X {
        pub i: String
    }
}

mod modfun {
    use super::{enumx, structx};

    pub fn longer(e: &enumx::X, s: &structx::X) -> usize {
        let s_len = s.i.len();
        
        let e_len = match e {
            enumx::X::Y(s) => s.len()
        };

        if e_len > s_len {
            e_len
        } else {
            s_len
        }
        
    }
}

// --- QUESTION 12 ---
pub fn maybesqrt(n: i32) -> Option<i32> {
    let f = n as f32;
    let res = f.sqrt();

    if res.is_nan() {
        None
    } else {
        Some(res as i32)
    }
}

// --- QUESTION 13 ---
pub struct Balance {
    pub amt: i32
}

impl Balance {
    pub fn withdraw(&self, n: i32) -> Option<i32> {
        let tmp = self.amt - n;
        if tmp < 0 {
            None
        } else {
            Some(tmp)
        }
    }
}

// --- QUESTION 14 ---
pub fn prevchar(c: char) -> i32 {
    if c == 'a' {
        return '`' as i32 
    } else {
        (c as i32) - 1
    }
}

pub fn replwithprev(s: &mut String) -> Result<String, ()> {
    let mut res = String::new();
    if s.contains('a') {
        return Err(())
    } else {
        for c in s.chars() {
            res.push(prev_char(c) as char);
        }
    }
    Ok(res)
}

// --- QUESTION 15 ---
#[derive(Debug)]
pub struct X {
    s: String,
    i: i32
}

#[derive(Debug)]
pub struct Y {
    z: bool,
    c: String
}

impl X {
    pub fn new() -> X {
        X {
            s: "xxx".to_string(),
            i: 10
        }
    }

    pub fn getstring(&mut self) -> String {
        std::mem::replace(&mut self.s, String::new())
    }
}

impl fmt::Display for X {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "S {}, I {}", self.s, self.i)
    }
}

impl Y {
    pub fn new() -> Y {
        Y {
            z: true,
            c: "op".to_string()
        }
    }

    pub fn getstring(&mut self) -> String {
        std::mem::replace(&mut self.c, String::new())
    }
}

impl fmt::Display for Y {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "B {}, C {}", self.z, self.c)
    }
}

pub fn swapstr(mut x: X, mut y:  Y) -> (X, Y) {
    std::mem::swap(&mut x.s, &mut y.c);
    (x, y)
}

// --- QUESTION 16 --
pub enum C {
    C1(i32, i32),
    C2(bool, String)
}

pub struct D {
    pub c: C,
    pub s: String
}

impl D {
    pub fn new() -> D {
        D {
            c: C::C1(0, 0),
            s: String::new()
        }
    }

    pub fn newfromc(c: C) -> D {
        let s = match &c {
            C::C2(_, ss) => ss.clone(),
            _ => String::from("not there")
        };

        D { c, s }
    }

    pub fn larger(&self) -> usize {
        match &self.c {
            C::C1(_, _) => self.s.len(),
            C::C2(_, ss) => {
                if self.s.len() > ss.len() {
                    self.s.len()
                } else {
                    ss.len()
                }
            },
        }
    }
}

// --- QUESTION 17 ---
// bo guarda lasciamo stare

// --- QUESTION 18 ---
pub fn veclengths(v: &Vec<String>) -> Vec<usize> {
    let mut res = Vec::new();

    for s in v.iter() {
        res.push(s.len());
    }

    res
}

// --- QUESTION 19 ---
pub fn remove_if_from_vector(v: &mut Vec<String>, length: usize) {
    if let Some(pos) = v.iter().position(|s| s.len() == length) {
        v.remove(pos);
    }
}

#[cfg(test)]
mod test_mt_2022 {
    use super::*;
    
    #[test]
    fn test_ex1() {
        let xx1 = XX::Y1(2,3);
        assert_eq!(data(xx1), 6);
        let xx1 = XX::Y2(String::from("test"));
        assert_eq!(data(xx1), 4);
    }

    #[test]
    fn test_ex2() {
        let z1 = Z::Y1(1,2);
        assert_eq!(maybelen(&z1), None);
        let z2 = Z::Y2(true, String::from("new"));
        assert_eq!(maybelen(&z2), Some(3));
    }

    #[test]
    fn test_ex3() {
        let ex = enumx::X::Y(String::from("test"));
        let sx = structx::X{i:String::from("asd")};
        assert_eq!(modfun::longer(&ex, &sx), 4);
        let ex = enumx::X::Y(String::from("asdasd"));
        assert_eq!(modfun::longer(&ex,&sx), 6);
    }

    #[test]
    fn test_ex4() {
        assert_eq!(maybesqrt(4), Some(2));
        assert_eq!(maybesqrt(-4), None);
    }

    #[test]
    fn test_ex5() {
        let b = Balance{amt: 100};
        assert_eq!(b.withdraw(10), Some(90));
        assert_eq!(b.withdraw(200), None);
    }

    #[test]
    fn test_ex6() {
        assert_eq!(prevchar('c'), 'b' as i32);
        assert_eq!(prevchar('a'), '`' as i32);
        assert_eq!(prevchar('z'), 'y' as i32);
        let mut s = String::from("Pign");
        assert_eq!(replwithprev(&mut s), Ok("Ohfm".to_string()));
        let mut s1 = String::from("pigna");
        assert_eq!(replwithprev(&mut s1), Err(()));
    }

    #[test]
    fn test_ex7() {
        let x = X::new();
        let y = Y::new();
        // println!("X {:?} - Y {:?}", x ,y);
        let (_x, _y) = swapstr(x, y);
        // println!("X {} - Y {}", x, y);
    }

    #[test]
    fn test_ex8() {
        let c1 = C::C1(0,1);
        let c2 = C::C2(true, String::from("no way jose"));
        let d1 = D::newfromc(c1);
        let d2 = D::newfromc(c2);
        assert_eq!(d1.larger(), 9);
        assert_eq!(d2.larger(), 11);
    }

    #[test]
    fn test_ex9() {
        let v1 = vec![String::from("some"); 12];
        let result = veclengths(&v1);
        assert_eq!(result.len(), 12);
        for l in result.iter() {
            assert_eq!(*l, 4);
        }

    }

    #[test]
    fn test_ex10() {
        let mut v = vec![String::from("what"); 5];
        v.push(String::from("now"));
        remove_if_from_vector(&mut v, 3);
        assert_eq!(v.len(), 5);
        for s in v.iter() {
            assert_eq!(*s, "what".to_string());
        }
    }

}