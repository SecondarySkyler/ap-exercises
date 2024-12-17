#![allow(unused)]

use std::{cell::RefCell, fmt::Debug, rc::Rc};

// Exercise 1
trait Double {
    fn double(&self) -> Self;
}

impl Double for i32 {
    fn double(&self) -> Self {
        self * 2
    }
}

impl Double for String {
    fn double(&self) -> Self {
        self.repeat(2)
    }
}

trait Triple {
    fn triple(&self) -> Self;
}

impl Triple for i32 {
    fn triple(&self) -> Self {
        self * 3
    }
}

impl Triple for String {
    fn triple(&self) -> Self {
        self.repeat(3)
    }
}

trait Sixfold {
    fn sixfold(&self) -> Self;
}

impl<T: Double + Triple> Sixfold for T {
    fn sixfold(&self) -> Self {
        self.double().triple()
    }
}

// Exercise 2
#[derive(Clone)]
struct Subtractor {
    v: u32
}

impl Subtractor {
    fn new(v: u32) -> Self {
        Subtractor { v }
    }
}

impl Iterator for Subtractor {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 0 {
            return  None;
        } else {
            let len = self.v.to_string().len() as u32;
            let first_digit = self.v.to_string().chars().next().unwrap().to_digit(10).unwrap();
            self.v = (self.v - first_digit) / len;
            return Some(self.v);
        }
    }
}

// Exercise 3
trait Printable {
    fn print(&self);
}

impl Printable for String {
    fn print(&self) {
        println!("{}", self)
    }
}

impl Printable for i32 {
    fn print(&self) {
        let sign = if *self < 0 { "negative".to_string() } else { "positive".to_string() };
        println!("{}({})", self.abs(), sign);
    }
}

fn get_vec() -> Vec<Box<dyn Printable>> { Vec::new() }

fn print_vec(v: Vec<Box<dyn Printable>>) {
    for el in v {
        el.print();
    }
}

// Exercise 4
#[derive(Debug)]
struct Circular<T: Debug + Clone> {
    next: Option<Rc<RefCell<Circular<T>>>>,
    to_end: usize,
    val: T
}

impl<T: Debug + Clone> Circular<T> {
    fn new(val0: T, val1: T) -> Rc<RefCell<Circular<T>>> {
        let rc_v0 = Rc::new(RefCell::new(Circular {
            next: None,
            to_end: 2,
            val: val0
        }));

        let rc_v1 = Rc::new(RefCell::new(Circular {
            next: Some(Rc::clone(&rc_v0)),
            to_end: 1,
            val: val1
        }));

        rc_v0.borrow_mut().next = Some(Rc::clone(&rc_v1));
        return rc_v0;
    }

    fn add_last(&mut self, val: T) {
        if self.to_end == 1 {
            let new_node = Rc::new(RefCell::new(Circular {
                next: self.next.take(),
                to_end: 1,
                val: val.clone()
            }));
            self.next = Some(Rc::clone(&new_node));
            self.to_end += 1;
        } else {
            self.to_end += 1;
            // call add_last on the next node
            let next = self.next.as_ref().unwrap();
            next.borrow_mut().add_last(val);
        }
    }

    fn cycles(s: &Rc<RefCell<Circular<T>>>, times: usize) {
        let len = s.borrow().to_end;
        Circular::cycle_under_the_hood(s, times * len);
    }

    fn cycle_under_the_hood(s: &Rc<RefCell<Circular<T>>>, times: usize) {
        if times == 0 {
            return;
        } else {
            println!("{:?}", s.borrow().val);
            Circular::cycle_under_the_hood(&Rc::clone(&s.borrow().next.as_ref().unwrap()), times - 1);
        }
    }
        
}


// Exercise 5 --- Who the F writes this exams? weird... btw
mod rack {
    pub mod router {
        use crate::midterm_2_18_11_2024_v2::rack::servers;

        pub fn router(addr: u32) {
            if addr >= 2 {
                println!("Not Found");
            } else {
                if addr == 0 {
                    println!("{:?}", servers::server0::Server::default())
                } else {
                    println!("{:?}", servers::server1::Server::default())
                }
            }
        }
    }

    mod servers {
        #[derive(Debug)]
        enum ServerStatus {
            On,
            Off,
            Standby
        }

        pub mod server0 {
            use super::ServerStatus;
            const S: ServerStatus = ServerStatus::Standby;

            #[derive(Debug)]
            pub struct Server {
                status: ServerStatus
            }

            impl Default for Server {
                fn default() -> Self {
                    Server { status: S }
                }
            }
        }

        pub mod server1 {
            use super::ServerStatus;
            const S: ServerStatus = ServerStatus::On;
            const P: &'static str = "s1";

            #[derive(Debug)]
            pub struct Server {
                status: ServerStatus,
                payload: &'static str
            }

            impl Default for Server {
                fn default() -> Self {
                    Server { status: S, payload: P }
                }
            }
        }
    }
}

// Exercise 6
// Ternary search tree with isthere() method
#[derive(Debug)]
struct Content {
    pub i: i32,
    pub s: String
}

impl Content {
    pub fn new(i: i32, s: String) -> Self {
        Content { i, s }
    }
}

impl PartialEq for Content {
    fn eq(&self, other: &Self) -> bool {
        self.s.len() == other.s.len()
    }
}

impl PartialOrd for Content {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.s.len().partial_cmp(&other.s.len())
    }
}

type TreeLink<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    left: TreeLink<T>,
    center: TreeLink<T>,
    right: TreeLink<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Node<T> {
        Node{ elem, left: None, center: None, right: None }
    }
}

#[derive(Debug)]
struct Tree<T> {
    root: TreeLink<T>
}

impl<T: PartialEq + PartialOrd> Tree<T> {
    fn new() -> Self {
        Tree { root: None }
    }

    fn add(&mut self, el: T) {
        let mut current_node = &mut self.root;
        while let Some(ref mut node) = current_node {
            if node.elem == el {
                current_node = &mut node.center;
            } else if node.elem < el {
                current_node = &mut node.right;
            } else {
                current_node = &mut node.left;
            }
        }
        *current_node = Some(Box::new(Node::new(el)));
    }

    fn isthere(&self, el: &T) -> Option<bool> {
        if self.root.is_none() {
            return None;
        } else {
            let mut curr = &self.root;
            while let Some(ref node) = curr {
                if node.elem == *el {
                    return Some(true);
                } else if node.elem < *el {
                    curr = &node.right;
                } else {
                    curr = &node.left;
                }
            }
            return None;
        }
    }
}

#[cfg(test)]
mod mt_2_18_11_2024_v2 {
    use std::rc;

    use super::*;

    #[test]
    fn test_sixfold() {
        assert_eq!(11.sixfold(), 66);
        assert_eq!(String::from("|").sixfold(), "||||||".to_string())
    }

    #[test]
    fn test_subtractor() {
        let w = Subtractor::new(42);
        assert_eq!(w.clone().collect::<Vec<u32>>(), vec![19, 9, 0]);

        let w = Subtractor::new(90);
        assert_eq!(w.clone().collect::<Vec<u32>>(), vec![40, 18, 8, 0]); 

        let w = Subtractor::new(123);
        assert_eq!(w.clone().collect::<Vec<u32>>(), vec![40, 18, 8, 0]);

        let w = Subtractor::new(65535);
        assert_eq!(w.clone().collect::<Vec<u32>>(), vec![13105, 2620, 654, 216, 71, 32, 14, 6, 0]); 
    }

    #[test]
    fn test_printable() {
        let mut v = get_vec();
        v.push(Box::new(358));
        v.push(Box::new("hi there".to_string()));
        v.push(Box::new(-87));
        v.push(Box::new("general kenobi".to_string()));
        v.push(Box::new(0));
        v.push(Box::new("test1".to_string()));
        v.push(Box::new("".to_string())); // maybe in this case we should avoid printing it
        v.push(Box::new("test2".to_string()));
        print_vec(v);
    }

    #[test]
    fn test_circular() {
        let rc = Circular::new(1, 2);
        // for i in 3..8 {
        //     rc.borrow_mut().add_last(i);
        // }
        println!("Displaying the circular list");
        Circular::cycles(&rc, 4);
    }

    #[test]
    fn test_server() {
        rack::router::router(0);
        rack::router::router(1);
        rack::router::router(2);
    }

    // Tests ternary tree
    #[test]
    fn test_content() {
        let e0 = Content::new(10,"asd".to_string());
        let e1 = Content::new(10,"who".to_string());
        let e2 = Content::new(11,"oneasd".to_string());

        assert_eq!(e1 == e2, false);
        assert_eq!(e1 == e0, true);
        assert_eq!(e1 < e2, true);
        assert_eq!(e1 > e2, false);
    }

    #[test]
    fn test_tree_content() {
        let mut t = Tree::new();
        let e1 = Content::new(10,"asd".to_string());
        let e2 = Content::new(11,"oneasd".to_string());
        let e3 = Content::new(8,"bhas".to_string());
        let e4 = Content::new(19,"xax".to_string());
        let e5 = Content::new(45,"gip".to_string());
        t.add(e1); t.add(e2); t.add(e3); t.add(e4); t.add(e5);
        println!("{:?}",t);
    }

    #[test]
    fn test_is_there() {
        let mut t = Tree{
            root: Some(Box::new(Node{
                elem: Content::new(10,"what".to_string()),
                left: Some(Box::new(Node{
                    elem: Content::new(11,"who".to_string()),
                    left: None,
                    center: Some(Box::new(Node{
                        elem: Content::new(19,"ten".to_string()),
                        left: None,
                        center: None,
                        right: None
                    })),
                    right: None
                })),
                center: Some(Box::new(Node{
                    elem: Content::new(15,"zips".to_string()),
                    left: None,
                    center: None,
                    right: None
                })),
                right: Some(Box::new(Node{
                    elem: Content::new(88,"whose".to_string()),
                    left: None,
                    center: None,
                    right: None
                }))
            }))
        };
        let e56 = Content::new(45,"gips".to_string());
        let e57 = Content::new(45,"naasdasdsad".to_string());
        assert_eq!(t.isthere(&e56), Some(true));
        assert_eq!(t.isthere(&e57), None);
    }

}