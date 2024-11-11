#![allow(unused)]

use std::str::Chars;

// Exercise 1
trait Nextable {
    fn gimme_next(&self) -> Option<Self> where Self: Sized;
}

impl Nextable for i32 {
    fn gimme_next(&self) -> Option<Self> where Self: Sized {
        Some(self + 1)
    }
}

impl Nextable for char {
    fn gimme_next(&self) -> Option<Self> where Self: Sized {
        Some((*self as u8 + 1) as char)
    }
}

fn printnext(el: impl Nextable + Sized + std::fmt::Debug) {
    println!("next of{:?} is {:?}", el, el.gimme_next());
}

// Exercise 2 
struct Wrapper {
    pub inner: String
}

impl Wrapper {
    fn iter(&self) -> ConsIter {
        ConsIter { iter: self.inner.chars() }
    }
}

struct ConsIter<'a> {
    iter: Chars<'a>
}

impl<'a> Iterator for ConsIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.iter.next() {
            Some(ch) => {
                let c = ch.to_ascii_lowercase();
                if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
                    self.next()
                } else {
                    Some(*ch)
                }
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod mt_2_22_01_2024 {
    use super::*;

    #[test]
    fn test_gimme_next() {
        assert_eq!(5.gimme_next(), Some(6));
        assert_eq!(11.gimme_next(), Some(12));
        assert_eq!('s'.gimme_next(), Some('t'));
        assert_eq!('f'.gimme_next(), Some('g'));
    }

    #[test]
    fn test_it() {
        let w = Wrapper { inner: "another day, another hangover!".to_string() };
        let w1 = Wrapper { inner: "AAAAAAAAAAAAOOOOUUUUUF!".to_string() };
        for (n, c) in w.iter().enumerate() {
            print!("{n}{c}");
        }
        
        let mut iw1 = w1.iter();
        assert_eq!(iw1.next(), Some('F'));
        assert_eq!(iw1.next(), Some('!'));
        assert_eq!(iw1.next(), None);
    }
}
