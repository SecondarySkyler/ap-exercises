#![allow(unused)]
use std::{cell::RefCell, fmt::{Debug, Formatter}, rc::Rc, usize };

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

// Exercise 4
type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
    len: i32,
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None, len: 0 }
    }

    fn size(&self) -> i32 {
        self.len
    }

    fn add(&mut self, elem: T, pos: i32) -> Result<(), String> {
        if self.len < pos {
            return Err("wrong position".to_string())
        } else if self.len == pos {
            self.append(elem);
            Ok(())
        } else {

            let mut current = &mut self.head;
            for _ in 0..pos {
                current = &mut current.as_mut().unwrap().next;
            }

            let new_node = Box::new(Node {
                elem,
                next: current.take(),
            });

            *current = Some(new_node);
            self.len += 1;
            Ok(())
        }
    }

    fn prepend(&mut self, elem: T) {
        match self.head.take() {
            Some(prev_head) => {
                self.head = Some(Box::new(
                    Node { elem, next: Some(prev_head) }
                ))
            }, 
            None => {
                self.head = Some(Box::new(
                    Node { elem, next: None }
                ))
            },
        }
    }

    fn append(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: None,
        });

        let mut current = &mut self.head;
        while let Some(ref mut node) = current {
            current = &mut node.next;
        }

        *current = Some(new_node);
        self.len += 1;
    }

    fn get(&self, pos: i32) -> Option<&T> {
        if pos >= self.len {
            return None
        } else {
            let mut current = &self.head;
            for _ in 0..pos {
                current = &current.as_ref().unwrap().next;
            }
            return Some(&current.as_ref().unwrap().elem);
        }
    }

}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

#[derive(Debug)]
pub struct Content {
    s : String, b : bool, i : i32,
}

impl Content {
    pub fn new_with(s:String, b:bool, i:i32) -> Content {
        return Content{s,b,i};
    }
}



// Exercise 5 ------------------------------------------------
type NodeRef<T> = Rc<RefCell<Node5<T>>>;
struct Node5<T> {
    inner_value: T,
    adjacent: Vec<NodeRef<T>>,
}
impl<T:Debug> Debug for Node5<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"iv: {:?}, adj: {}", self.inner_value, self.adjacent.len())
    }
}

#[derive(Debug)]
struct Graph<T> {
    nodes: Vec<NodeRef<T>>,
}
pub trait SameBool{
    fn samebool(&self, other:&Self)->bool;
}
#[derive(Debug)]
pub struct Content5 {
    pub i:i32,
    pub b:bool
}
impl Content5 {
    pub fn new_with(i: i32, b: bool) -> Content5 {
        Content5 { i, b }
    }
}

impl PartialEq for Content5 {
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i
    }
}

impl PartialOrd for Content5 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.i.cmp(&other.i))
    }
}

impl SameBool for Content5 {
    fn samebool(&self, other:&Self)->bool {
        self.b == other.b
    }
}

impl<T: SameBool + PartialOrd> Graph<T> {
    fn new() -> Self {
        Graph { nodes: vec![] }
    }

    fn add_node(&mut self, elem: T) {
        let mut new_node = Rc::new(RefCell::new(
            Node5 {
                inner_value: elem, adjacent: vec![]
            }
        ));

        for node in &self.nodes {
            if node.borrow().inner_value < new_node.borrow_mut().inner_value {
                new_node.borrow_mut().adjacent.push(Rc::clone(&node));
            }

            if node.borrow().inner_value.samebool(&new_node.borrow().inner_value) {
                node.borrow_mut().adjacent.push(Rc::clone(&new_node));
            }
        }

        self.nodes.push(new_node);
    }
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

    #[test]
    fn test_new_ex4() {
        let list: List<i32> = List::new();
        assert_eq!(list.len, 0);

        let list2 = List{ head: Some(Box::new(Node{ elem: 4, next: None })), len: 1 };
        assert_eq!(list2.size(), 1);
    }

    #[test]
    fn test_get_ex4() {
        let list: List<i32> = List::new();
        assert_eq!(list.get(0), None);
        assert_eq!(list.get(1), None);

        let list2 = List{ head: Some(Box::new(Node{ elem: 4, next: None })), len: 1 };
        assert_eq!(list2.get(0), Some(&4));
    }

    #[test]
    fn test_add_ex4() {
        let elem0 = Content::new_with("test".to_string(),true,2);
        let elem1 = Content::new_with("what".to_string(),true,2);
        let elem2 = Content::new_with("this".to_string(),false,8);
        let elem3 = Content::new_with("dope".to_string(),true,18);
        let mut l : List<Content> = List::new();
        
        assert_eq!(l.add(elem0, 1), Err("wrong position".to_string()));

        assert_eq!(l.add(elem1, 0), Ok(()));
        assert_eq!(l.size(), 1);

        assert_eq!(l.add(elem2, 0), Ok(()));
        assert_eq!(l.add(elem3, 0), Ok(()));
        assert_eq!(l.size(), 3);
    }

    #[test]
    fn test_append_ex4() {
        let mut l : List<i32> = List::new();
        
        l.append(0);
        assert_eq!(l.get(0), Some(&0));
        l.append(1);
        l.append(2);
        assert_eq!(l.get(2), Some(&2));
    }

    #[test]
    fn test_eq() {
        let el1 = Content5{i:10, b:true};
        let el2 = Content5{i:11, b:true};
        let el3 = Content5{i:11, b:false};
        assert_eq!(el1<el2,true);
        assert_eq!(el2<el1,false);
        assert_eq!(el2==el3,true);
    }

    #[test]
    fn test_samebool() {
        let el1 = Content5{i:10, b:true};
        let el2 = Content5{i:11, b:true};
        let el3 = Content5{i:11, b:false};
        assert_eq!(el1.samebool(&el2), true);
        assert_eq!(el1.samebool(&el3), false);
    }

    #[test]
    fn test_graph() {
        let el1 = Content5{i:10, b:true};
        let el2 = Content5{i:11, b:true};
        let el3 = Content5{i:11, b:false};
        let mut g: Graph<Content5> = Graph::new();

        // g.add_node(el1);
        // g.add_node(el2);
        // println!("{:?}", g);

        // let el1 = Content5{i:10, b:true};
        // let el2 = Content5{i:8, b:false};
        // g.add_node(el1);
        // g.add_node(el2);
        // println!("{:?}", g);
        // let mut el1 = Content5{i:10, b:true};
        // let mut el2 = Content5{i:11, b:true};
        // let mut el3 = Content5{i:12, b:true};
        // let mut g = Graph::new();
        // println!("{:?}",g);
        // g.add_node(el1);
        // println!("{:?}",g);
        // g.add_node(el2);
        // println!("{:?}",g);
        // g.add_node(el3);
        // println!("{:?}",g);

        let mut el1 = Content5{i:10, b:true};
        let mut el2 = Content5{i:9, b:false};
        let mut el3 = Content5{i:8, b:true};
        let mut g = Graph::new();
        println!("{:?}",g);
        g.add_node(el1);
        println!("{:?}",g);
        g.add_node(el2);
        println!("{:?}",g);
        g.add_node(el3);
        println!("{:?}",g);

    }
}
