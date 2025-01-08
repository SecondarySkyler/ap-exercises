#![allow(unused)]

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

fn printdouble(e: impl Doublable + std::fmt::Debug) {
    println!("doubling {:?} is {:?}", e, e.gimme_double());
}

// Exercise 2
struct Wrapper {
    pub v: Vec<i32>
}

impl Wrapper {
    fn iter(&self) -> WI {
        WI { container: self, index: 0}
    }
}

struct WI<'a> {
    container: &'a Wrapper,
    index: usize
}

impl<'a> Iterator for WI<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index< self.container.v.len()   {
            let num = Some(self.container.v[self.index]);
            self.index += 2;
            num
        } else {
            None
        }
    }
}

// Exercise V2
struct WrapperV2 {
    v: Vec<String>
}

impl WrapperV2 {
    fn iter(&self) -> WIV2 {
        WIV2 { container: self, index: 0 }
    }
}

struct WIV2<'a> {
    container: &'a WrapperV2,
    index: usize
}

impl<'a> Iterator for WIV2<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.container.v.len() {
            let current_len = &self.container.v[self.index];
            self.index += 1;
            Some(current_len.len())
        } else {
            None
        }
    }
}


// Exercise 3
fn basicbox_inc(v: Vec<String>) -> Vec<Box<usize>> {
    let mut res: Vec<Box<usize>> = Vec::new();
    for s in v {
        res.push(Box::new(s.len() + 1));
    }
    res
}

fn basicbox_inc_v2(v: Vec<String>) -> Vec<Box<usize>> {
    v.iter().map(|s| Box::new(s.len() + 1)).collect()
}

// Exercise 4
#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
    len: i32,
}
type Link<T> = Option<Box<Node<T>>>;
#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T: PartialOrd + PartialEq> Node<T> {
    fn add(&mut self, e: T) {
        if self.elem >= e {
            let mut new_node = Node { elem: e, next: None };
            std::mem::swap(self, &mut new_node);
            self.next = Some(Box::new(new_node));
            return;
        }

        match self.next {
            None => {
                self.next = Some(Box::new(Node { elem: e, next: None }));
                return;
            },
            Some(ref mut node) => { node.add(e); }
        }
    }
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

impl PartialOrd for Content {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.i.cmp(&other.i))
    }
}

impl PartialEq for Content {
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i
    }
}

impl<T: PartialOrd + PartialEq> List<T> {
    fn new() -> Self {
        Self {head: None, len: 0}
    }

    fn add(&mut self, e: T) {
        let new_node = Box::new(Node {
            elem: e,
            next: None,
        });

        if self.len == 0 {
            self.head = Some(new_node);
            self.len += 1;
            return;
        }

        if self.head.as_ref().unwrap().elem > new_node.elem {
            let next = self.head.take();
            self.head = Some(new_node);
            self.head.as_mut().unwrap().next = next;
            self.len += 1;
            return;
        }

        let mut current = &mut self.head;
        while let Some(ref mut node) = current  {
            if node.elem < new_node.elem {
                // Here I can check whether the next node is greater than the new node or none, in both cases I can insert the new node
                if node.next.is_none() || node.next.as_ref().unwrap().elem > new_node.elem {
                    let next = node.next.take();
                    node.next = Some(new_node);
                    node.next.as_mut().unwrap().next = next;
                    self.len += 1;
                    return;
                } else {
                    current = &mut current.as_mut().unwrap().next;
                }
            }
            
        }
    }

    fn add_rec(&mut self, e: T) {
        self.len += 1;
        match self.head {
            None => {
                self.head = Some(Box::new(Node { elem: e, next: None, }));
                return;
            },
            Some(ref mut node) => { node.add(e) }
        }
    }

    fn get(&self, pos: i32) -> Option<&T> {
        if pos >= self.len {
            None
        } else {
            let mut current = &self.head;
            for _ in 0..pos {
                current = &current.as_ref().unwrap().next;
            }
            Some(&current.as_ref().unwrap().elem)
        }
    }
}


// Exercise 4 V2
#[derive(Debug)]
pub struct ContentV2{
        pub i:i32,
        pub s:String
}

impl ContentV2 {
    pub fn new(i: i32, s: String) -> ContentV2 {
        ContentV2 { i, s }
    }
}

impl PartialOrd for ContentV2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.s.len().cmp(&other.s.len()))
    }
}

impl PartialEq for ContentV2 {
    fn eq(&self, other: &Self) -> bool {
        self.s.len() == other.s.len()
    }
}

type TreeLink<T> = Option<Box<NodeV2<T>>>;
#[derive(Debug)]
struct NodeV2<T> {
    elem: T,
    left: TreeLink<T>,
    center: TreeLink<T>,
    right: TreeLink<T>,
}

impl<T> NodeV2<T>{
    pub fn new(elem:T) -> NodeV2<T> {
        NodeV2{elem, left:None, center:None, right:None}
    }
}


#[derive(Debug)]
pub struct Tree<T> {
    root: TreeLink<T>,
    size : i32,
}

impl<T: PartialEq + PartialOrd> Tree<T> {
    fn new() -> Self {
        Tree { root: None, size: 0 }
    }

    fn add_node(&mut self, el: T) {
        let mut copy = &mut self.root;
        while let Some(ref mut node) = copy {
            if node.elem == el {
                copy = &mut node.center;
            } else if node.elem < el {
                copy = &mut node.right;
            } else {
                copy = &mut node.left;
            }
        }
        *copy = Some(Box::new(NodeV2::new(el)));
        self.size += 1;
    }

    
}

#[cfg(test)]
mod mt_2_24_01_2023 {
    use super::*;

    #[test]
    fn test_gimme_double() {
        assert_eq!(5.gimme_double(), 10);
        assert_eq!("what".to_string().gimme_double(), "whatwhat".to_string());
    }

    #[test]
    fn test_wi() {
        let w = Wrapper{v:vec![1,2,3,4,5,6,7,8]};
        let mut iw = w.iter();
        assert_eq!(iw.next(), Some(1));
        assert_eq!(iw.next(), Some(3));
        assert_eq!(iw.next(), Some(5));
        assert_eq!(iw.next(), Some(7));
        assert_eq!(iw.next(), None);
    }

    #[test]
    fn test_wiv2() {
        let w = WrapperV2{v:vec!["nope".to_string(),"who".to_string(),"gimme".to_string()]};
        let mut iw = w.iter();
        assert_eq!(iw.next().unwrap(), 4);
        assert_eq!(iw.next().unwrap(), 3);
        assert_eq!(iw.next().unwrap(), 5);
    }

    #[test]
    fn test_basic_box_inc() {
        let s = vec!["asd".to_string(), "where".to_string(), "what".to_string()];
        let res = basicbox_inc(s);
        assert_eq!(*res[0], 4);
        assert_eq!(*res[1], 6);
        assert_eq!(*res[2], 5);

        let s2 = vec!["who".to_string(), "whose".to_string(), "noway".to_string()];
        let res2 = basicbox_inc(s2);
        assert_eq!(*res2[0], 4);
        assert_eq!(*res2[1], 6);
        assert_eq!(*res2[2], 6);
    }

    #[test]
    fn test_basic_box_inc_v2() {
        let s = vec!["asd".to_string(), "where".to_string(), "what".to_string()];
        let res = basicbox_inc_v2(s);
        assert_eq!(*res[0], 4);
        assert_eq!(*res[1], 6);
        assert_eq!(*res[2], 5);

        let s2 = vec!["who".to_string(), "whose".to_string(), "noway".to_string()];
        let res2 = basicbox_inc_v2(s2);
        assert_eq!(*res2[0], 4);
        assert_eq!(*res2[1], 6);
        assert_eq!(*res2[2], 6);
    }

    #[test]
    fn test_list() {
        let elem1 = Content::new_with("what".to_string(),true,2);
        let elem2 = Content::new_with("this".to_string(),false,18);
        let elem3 = Content::new_with("dope".to_string(),false,5);
        let mut l : List<Content> = List::new();
        l.add(elem1);
        l.add(elem2);
        l.add(elem3);
        let elem4 = Content::new_with("nope".to_string(),false,1);
        l.add(elem4);
        println!("{:?}",l);
    }

    #[test]
    fn test_list_2() {
        let mut l: List<i32> = List::new();
        l.add(2);
        l.add(3);
        l.add(18);
        println!("{:?}", l);
    }

    #[test]
    fn test_list_3() {
        let mut l: List<i32> = List::new();
        l.add(18);
        l.add(2);
        l.add(3);
        println!("{:?}", l);
    }

    #[test]
    fn test_list_add_rec_() {
        let mut l: List<i32> = List::new();
        l.add_rec(2);
        l.add_rec(18);
        l.add_rec(5);
        l.add_rec(1);
        println!("{:?}", l);
    }

    #[test]
    fn test_list_add_rec_2() {
        let mut l: List<i32> = List::new();
        l.add_rec(2);
        l.add_rec(3);
        l.add_rec(18);
        println!("{:?}", l);
    }

    #[test]
    fn test_list_add_rec_3() {
        let mut l: List<i32> = List::new();
        l.add_rec(18);
        l.add_rec(2);
        l.add_rec(3);
        println!("{:?}", l);
    }

    #[test]
    fn test_weird_tree() {
        let mut t = Tree::new();
        let e1 = ContentV2::new(10,"asd".to_string());
        let e2 = ContentV2::new(11,"oneasd".to_string());
        let e3 = ContentV2::new(8,"bhas".to_string());
        let e4 = ContentV2::new(19,"xax".to_string());
        let e5 = ContentV2::new(45,"gip".to_string());
        t.add_node(e1);
        t.add_node(e2); 
        t.add_node(e3); 
        t.add_node(e4); 
        t.add_node(e5);
        println!("{:?}",t);
    }
            
}