#![allow(unused)]

// Implementation of a Doubly Linked List
use std::{borrow::BorrowMut, cell::RefCell, fmt::{Debug, Display}, rc::Rc};

type OptNode<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    data: T,
    prev: OptNode<T>,
    next: OptNode<T>
}

struct List<T> {
    size: usize,
    head: OptNode<T>,
    tail: OptNode<T>
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node Val: {}", self.data)
    }
}

impl<T> Node<T> {
    fn new(element: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node { data: element, prev: None, next: None }))
    }
}

impl<T: Display + PartialEq> PartialEq for List<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        } else {
            todo!()
        }
    }
}

impl<T: Display + PartialEq> Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<T: Display + PartialEq> List<T> {
    fn new() -> Self {
        List { size: 0, head: None, tail: None }
    }

    fn print_list(&self) {}
    fn push(&mut self, element: T) {
        let new_node = Node::new(element);

        // First of all, get rid of the Option
        match self.head.take() {
            // take() takes the value out of the option if there is some, otherwise return None
            Some(prev_head) => {
                // prev_head.borrow_mut().prev = Some(new_node.clone());
                // new_node.borrow_mut().next = Some(prev_head.clone());
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




    fn pop(&mut self) -> Option<T> { todo!()}
    fn push_back(&mut self, element: T) {}
    fn pop_back(&mut self) -> Option<T> { todo!()}

}