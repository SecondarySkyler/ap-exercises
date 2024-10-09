#![allow(unused)]
use std::result;

pub fn find_equal<'a>(s1: &'a str, s2: &'a str) -> Option<(&'a str, &'a str)> {
    for i in 0..s1.len() - 1{
        let slice1 = &s1[i..i + 2]; 

        if let Some(j) = s2.find(slice1) {
            return Some((slice1, &s2[j..j + 2]));
        }
    }
    None
}

// fn random_letter() -> char{
//     let mut n = rand::random::<u8>();
//     n = n % 26 + 'a' as u8;
//     n as char
// }
// fn random_string(len: usize) -> String{
//     let mut s = String::with_capacity(len);
//     for _ in 0 .len{
//         s.push(random_letter());
//     }
//     s
// }

// pub fn lucky_slice(input_str: &str) -> Option<&str> {
//     let second_str = random_string(input_str.len());
//     let result = find_equal(input_str, &second_str);
//     if result.is_none(){
//         return None;
//     }
//     return Some(result.unwrap().0);
// }

// Exercise 2
pub struct Person<'a> {
    pub name: String,
    pub father: Option<&'a Person<'a>>,
    pub mother: Option<&'a Person<'a>>
}

impl<'a> Person<'a> {
    pub fn new(name: String, father: Option<&'a Person<'a>>, mother: Option<&'a Person<'a>>) -> Self {
        Self {name, father, mother}
    }

    fn rec_find_relative<'b> (&'b self, generations: u32, relatives: &mut Vec<&'b Self>) {
        relatives.push(self);

        if generations > 0 {
            if let Some(mother) = self.mother {
                mother.rec_find_relative(generations - 1, relatives);
            }

            if let Some(father) = self.father {
                father.rec_find_relative(generations - 1, relatives);
            }
        }
    }
    pub fn find_relatives(&self, generations: u32) -> Vec<&Person> {
        let mut relatives = Vec::new();
        self.rec_find_relative(generations, &mut relatives);
        relatives
    }

    fn rec_find_roots<'b>(&'b self, roots: &mut Vec<&'b Self>) {
        match (self.mother, self.father) {
            (None, None) => roots.push(self),
            (None, Some(f)) => {
                f.rec_find_roots(roots);
                roots.push(self);
            },
            (Some(m), None) => {
                m.rec_find_roots(roots);
                roots.push(self);
            },
            (Some(m), Some(f)) => {
                m.rec_find_roots(roots);
                f.rec_find_roots(roots);
            },
        }
    }
    pub fn find_roots(&self) -> Vec<&Person> {
        let mut roots = Vec::new();
        self.rec_find_roots(&mut roots);
        roots
    }
}

#[cfg(test)]
mod test4 {
    use super::*;

    #[test]
    fn test_find_equal() {
        assert_eq!(find_equal("hello", "world"), None);
        assert_eq!(find_equal("hello", "hell"), Some(("he", "he")));
    }
}