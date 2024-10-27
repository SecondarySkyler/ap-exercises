#![allow(unused)]

use std::{cell::RefCell, fmt::Display, rc::Rc};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone)]
struct Node<T> {
    data: T,
    next: Link<T>
}

impl<T> Node<T> {
    fn new(element: T) -> Node<T> {
        Node {data: element, next: None}
    }
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

struct LinkedList<T> {
    size: usize,
    head: Link<T>,
    tail: Link<T>
}

impl<T: Clone> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList { size: 0, head: None, tail: None }
    }

    fn insert_front(&mut self, element: T) {
        let new_node = Rc::new(RefCell::new(Node::new(element)));
        match self.head.take() {
            Some(prev_head) => {
                new_node.borrow_mut().next = Some(prev_head.clone());
                self.head = Some(new_node.clone());
                self.size += 1;
            },
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node.clone());
                self.size += 1;
            },
        }
    }

    fn insert_back(&mut self, element: T) {
        let new_node = Rc::new(RefCell::new(Node::new(element)));
        match self.tail.take() {
            Some(prev_tail) => {
                prev_tail.borrow_mut().next = Some(new_node.clone());
                self.tail = Some(new_node.clone());
                self.size += 1;
            },
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node.clone());
                self.size += 1;
            },
        }
    }

    fn pop_head(&mut self) -> Option<T> {
        match self.head.take() {
            Some(head) => {
                self.head = head.borrow().next.clone();
                return Some(head.borrow().data.clone())
            },
            None => return  None,
        }
    }

}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current_head = self.head.clone();
        while let Some(node) = current_head {
            write!(f, "{}", node.borrow().data);
            current_head = node.borrow().next.clone();
        }
        Ok(())
    }
}

#[cfg(test)]
mod test_ll {
    use super::*;

    #[test]
    fn test_size() {
        let mut list = LinkedList::<i32>::new();
        list.insert_front(5);
        assert_eq!(list.size, 1);
    }

    #[test]
    fn test_front_insert() {
        let mut list = LinkedList::<i32>::new();
        list.insert_front(1);
        list.insert_front(2);
        list.insert_front(3);
        list.insert_front(4);
        assert_eq!(list.head.unwrap().borrow().data, 4);
        assert_eq!(list.tail.unwrap().borrow().data, 1);
    }

    #[test]
    fn test_back_insert() {
        let mut list = LinkedList::<i32>::new();
        list.insert_back(1);
        list.insert_back(2);
        list.insert_back(3);
        list.insert_back(4);
        assert_eq!(list.head.unwrap().borrow().data, 1);
        assert_eq!(list.tail.unwrap().borrow().data, 4);
    }

    #[test]
    fn test_pop_head() {
        let mut list = LinkedList::<i32>::new();
        list.insert_back(1);
        list.insert_back(2);
        list.pop_head();
        assert_eq!(list.head.as_ref().unwrap().borrow().data, 2);
        list.pop_head();
        assert_eq!(list.pop_head(), None);
    }
}