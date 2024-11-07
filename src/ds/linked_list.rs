#![allow(unused)]

use std::{borrow::Borrow, cell::RefCell, fmt::Display, rc::Rc};

type Link<T> = Option<Box<Node<T>>>;

#[derive(Clone, Debug)]
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
        let mut new_node = Box::new(Node::new(element));

        match self.head.take() {
            Some(prev_head) => {
                new_node.next = Some(prev_head);
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


    fn pop_head(&mut self) -> Option<T> {
        match self.head.take() {
            Some(current_head) => {
                self.head = current_head.next;
                self.size -= 1;
                Some(current_head.data)
            },
            None => None,
        }
    }

    fn iter(&self) -> LLIter<T> {
        LLIter { head: self.head.as_deref() }
    }

    fn iter_mut(&mut self) -> LLMutIter<T> {
        todo!()
    }

}

struct LLIter<'a, T> {
    head: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for LLIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.head.map(|node| {
            self.head = node.next.borrow().as_deref();
            &node.data
        })
        
    }
}

struct LLMutIter<'a, T> {
    list: &'a mut LinkedList<T>
}

impl<'a, T> Iterator for LLMutIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current_head = &self.head;
        while let Some(node) = current_head {
            write!(f, "{}", node.data);
            current_head = &node.next;
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
        assert_eq!(list.head.unwrap().data, 4);
        assert_eq!(list.tail.unwrap().data, 1);
    }

    #[test]
    fn test_pop_head() {
        let mut list = LinkedList::<i32>::new();
        list.insert_front(1);
        list.insert_front(2);
        list.pop_head();
        assert_eq!(list.head.as_ref().unwrap().data, 1);
        list.pop_head();
        assert_eq!(list.pop_head(), None);
    }

    #[test]
    fn test_iter() {
        let mut list = LinkedList::<i32>::new();
        list.insert_front(1);
        list.insert_front(2);
        list.insert_front(3);
        list.insert_front(4);
        let mut list_it = list.iter();
        assert_eq!(list_it.next(), Some(&4));
        assert_eq!(list_it.next(), Some(&3));
        assert_eq!(list_it.next(), Some(&2));
        assert_eq!(list_it.next(), Some(&1));
    }
}