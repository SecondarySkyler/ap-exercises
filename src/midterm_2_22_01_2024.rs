#![allow(unused)]

use std::{cell::RefCell, rc::Rc, str::Chars};

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

// Exercise 3
fn basicbox_sum(v: Vec<String>) -> Vec<Box<usize>> {
    let mut result: Vec<Box<usize>> = Vec::new();
    let mut tot_sum = 0;

    for s in v {
        result.push(Box::new(s.len()));
        tot_sum += s.len();
    }
    result.push(Box::new(tot_sum));

    return result
}

// Exercise 4
#[derive(Debug)]
struct MasterClock {
    clock_cycle: Rc<RefCell<usize>>
}

impl MasterClock {
    fn new() -> Self {
        MasterClock { clock_cycle: Rc::new(RefCell::new(0)) }
    }

    fn tick(&mut self) {
        *self.clock_cycle.borrow_mut() += 1;
    }

    fn get_slave(&self) -> SlaveClock {
        SlaveClock { master_clock: Rc::clone(&self.clock_cycle) }
    }
}

#[derive(Debug)]
struct SlaveClock {
    master_clock: Rc<RefCell<usize>>
}

impl SlaveClock {
    fn get_clock(&self) -> usize {
        *self.master_clock.borrow()
    }
}

mod finance {
    type Wallet1 = wallet_1::Wallet;
    type Wallet2 = wallet_2::Wallet;

    pub mod wallet_1 {
        #[derive(Debug)]
        pub struct Wallet {
            pub euro: f32
        }
    }

    pub mod wallet_2 {
        #[derive(Debug)]
        pub struct Wallet {
            pub euro: u32,
            pub cents: u8
        }
    }

    pub fn compare_wallet(w1: &Wallet1, w2: &Wallet2) -> bool {
        if w1.euro > w2.euro as f32 {
            return true;
        } else if w1.euro == w2.euro as f32 && w2.cents > 0 {
            return false;
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod mt_2_22_01_2024 {
    use finance::compare_wallet;

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

    #[test]
    fn test_bb_sum() {
        let s = vec!["asd".to_string(), "where".to_string(), "what".to_string()];
        let r1 = basicbox_sum(s);
        assert_eq!(r1, vec![Box::new(3), Box::new(5), Box::new(4), Box::new(12)]);

        let v = vec!["nope".to_string(), "game".to_string(), "bananas".to_string()];
        let r2 = basicbox_sum(v);
        assert_eq!(r2, vec![Box::new(4), Box::new(4), Box::new(7), Box::new(15)]);
    }

    #[test]
    fn test_clock() {
        let mut mc = MasterClock::new();
        assert_eq!(mc.clock_cycle, Rc::new(RefCell::new(0 as usize)));

        mc.tick();
        mc.tick();
        assert_eq!(mc.clock_cycle, Rc::new(RefCell::new(2 as usize)));

        let mut mc1 = MasterClock::new();
        let sc1 = mc1.get_slave();

        mc1.tick();
        mc1.tick();
        mc1.tick();

        assert_eq!(sc1.get_clock(), 3);
        let sc2 = mc1.get_slave();

        mc1.tick();
        mc1.tick();

        assert_eq!(sc1.get_clock(), 5);
        assert_eq!(sc2.get_clock(), 5);
    }

    #[test]
    fn test_wallet() {
        let w1 = finance::wallet_1::Wallet{ euro: 100. };
        let w2 = finance::wallet_2::Wallet{ euro: 90, cents: 50 };

        assert_eq!(compare_wallet(&w1, &w2), true);
    }
}
