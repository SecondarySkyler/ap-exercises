#![allow(dead_code)]

use std::collections::HashMap;

// --- QUESTION 9 ---
#[derive(Debug)]
pub enum A {
    A1(char, char),
    A2(i32, i32, i32)
}

#[derive(Debug)]
pub enum B {
    B1(i32, i32),
    B2(String)
}

pub fn bfroma(a: A) -> B {
    match a {
        A::A1(c1, c2) => {
            return B::B1(c1 as i32, c2 as i32)
        },
        A::A2(f, s, t) => {
            let mut res: String = String::new();
            res.push_str(&f.to_string());
            res.push_str("-");
            res.push_str(&s.to_string());
            res.push_str("-");
            res.push_str(&t.to_string());
            return B::B2(res)
        }
    }
}

pub enum E {
    A(String),
    B(bool)
}
pub enum F {
    F1(String),
    F2(i32)
}

impl E {
    pub fn count_vowels(&self) -> i32 {
        let mut count = 0;
        match self {
            E::A(s) => {
                for c in s.chars() {
                    if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
                        count += 1;
                    }
                }
                count
            },
            E::B(_) => 0,
        }
    }
}

impl F {
    pub fn new() -> F {
        F::F1("hello".to_string())
    }

    pub fn calculation(&self) -> usize {
        match self {
            F::F1(s) => s.len(),
            F::F2(val) => *val as usize,
        }
    }
}

// --- QUESTION 11 ---
pub fn print_n(opt: Option<i32>) {
    match opt {
        None => println!("Error"),
        Some(x) => {
            for _ in 0..x {
                println!("{}", x);
            }
        }
    }
}



// --- QUESTION 12 ---
struct Balance {
    pub amt: i32,
    pub active: bool
}

impl Balance {
    pub fn maybericher(&self, other: Balance) -> Option<bool> {
        if !self.active || !other.active  {
            return None
        } else {
            if self.amt > other.amt {
                return Some(true)
            } else {
                Some(false)
            }
        }

        // also "return Some(self.amt > other.amt)" works 

    }
}

// --- QUESTION 13 ---
struct G {
    pub x: i32,
    pub y: i32,
}

impl G {
    pub fn new(x: i32, y: i32) -> G {
        G { x, y }
    }

    pub fn square(&self) -> Result<i32, ()> {
        let sq = self.y.pow(2);
        if self.x == sq {
            Ok(self.x)
        } else {
            Err(())
        }
    }
}

// --- QUESTION 14 ---
// same as midterm of 2022

// --- QUESTION 15 ---
#[derive(Debug)]
pub struct L {
    pub s: String,
    pub n: i32
}
#[derive(Debug)]
pub struct M {
    pub s: String,
    pub n: f64
}
impl L {
    fn new() -> L {
        L { s: "hello".to_string(), n: 0 }
    }

    fn new_with_params(s: String, n: i32) -> L {
        L { s, n }
    }
}

impl M {
    fn new() -> M {
        M { s: "hello".to_string(), n: 0. }
    }

    fn new_with_params(s: String, n: f64) -> M {
        M { s, n }
    }
}

pub fn swap_string(l: &mut L, m: &mut M) {
    std::mem::swap(&mut l.s, &mut m.s);
}

// --- QUESTION 16 ---
pub fn neighbour(v: &Vec<String>, i: usize) -> Result<String, ()> {
    if i == v.len() - 1 { return Err(())} else {
        let mut res = String::new();
        res.push_str(&v[i]);
        res.push_str(&v[i+1]);
        Ok(res)
    }
}

// --- QUESTION 17 ---
pub fn remove_element(v: &mut Vec<Option<i32>>) {
    let pos= v.iter().position(|opt| {
        match opt {
            Some(i) => {
                return i % 2 == 1
            },
            None => true,
        }
    });

    if pos.is_some(){
        v.remove(pos.unwrap());
    }
}

// --- QUESTION 18 ---
pub fn hashandhash(h1: &mut HashMap<i32, String>, h2: &mut HashMap<String, i32>) {
    *h2 = h2.clone().into_iter().filter(|(k, _)| {
        return h1.get(&(k.len() as i32)).is_none()
    }).collect();
}

// --- QUESTION 19 ---
pub fn unique(mut h: HashMap<i32, String>, i: i32) -> Option<HashMap<i32, String>> {
    if let Some(_) = h.get(&i) {
        return None
    } else {
        let key = h.len() as i32;
        let val = (0..i).map(|_| "X").collect::<String>();
        h.insert(key, val);
        return Some(h)
    }
}



#[cfg(test)]
mod test_mt_2023 {
    use super::*;

    #[test]
    fn test_ex9() {
        let _a1 = A::A1('a', 'b');
        let _a2 = A::A2(1, 2, 3);
        // println!("B2: {:?}, B1: {:?}", bfroma(a2), bfroma(a1));
    }

    #[test]
    fn test_ex10() {
        let e1 = E::A("hello".to_string());
        let e2 = E::B(true);
        assert_eq!(e1.count_vowels(), 2);
        assert_eq!(e2.count_vowels(), 0);

        let f1 = F::F1("hello".to_string());
        let f2 = F::F2(20);
        assert_eq!(f1.calculation(), 5);
        assert_eq!(f2.calculation(), 20);
    }

    #[test]
    fn test_ex11() {
        let o1: Option<i32> = None;
        let o2 = Some(3);
        print_n(o1);
        print_n(o2);
    }

    #[test]
    fn test_ex12() {
        let b1 = Balance{ amt: 100, active: true};
        let b2 = Balance{ amt: 0, active: true};
        assert_eq!(b1.maybericher(b2), Some(true));

        let b3 = Balance{ amt: 0, active: true};
        let b4 = Balance{ amt: 100, active: true};
        assert_eq!(b3.maybericher(b4), Some(false));

        let b5 = Balance{ amt: 0, active: false};
        let b6 = Balance{ amt: 100, active: true};
        assert_eq!(b5.maybericher(b6), None);

        let b5 = Balance{ amt: 0, active: true};
        let b6 = Balance{ amt: 100, active: false};
        assert_eq!(b5.maybericher(b6), None);
    }

    #[test]
    fn test_ex13() {
        let g = G::new(4, 2);
        assert_eq!(g.square(), Ok(4));

        let g1 = G::new(4, 3);
        assert_eq!(g1.square(), Err(()));
    }

    #[test]
    fn test_ex15() {
        let mut l = L::new_with_params("hello".to_string(), 0);
        let mut m = M::new_with_params("world".to_string(), 0.0);
        swap_string(&mut l, &mut m);
        println!("{:?} {:?}", l, m);
    }

    #[test]
    fn test_ex16() {
        let v = vec!["hello".to_string(), "world".to_string()];
        assert_eq!(neighbour(&v, 0), Ok("helloworld".to_string()));
        assert_eq!(neighbour(&v, 1), Err(()));
    }

    #[test]
    fn test_ex17() {
        let mut v = vec![Some(1), Some(2), None, Some(3)];
        remove_element(&mut v);
        println!("{:?}", v);
    }

    #[test]
    fn test_ex19() {
        let mut h = HashMap::new();
        h.insert(2, "what".to_string());
        h.insert(3, "what3".to_string());

        // let ret = unique(h, 3);
        // assert_eq!(ret, None);

        let ret2 = unique(h, 4);
        // here the length is 2 because we overwrite the entry with key == 2
        assert_eq!(ret2.unwrap().len(), 2); 
    }
}