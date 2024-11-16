#![allow(unused)]

type TreeLink<T> = Option<Box<Node<T>>>;
struct Node<T> {
    element: T,
    left: TreeLink<T>,
    right: TreeLink<T>
}

impl<T> Node<T> {
    fn new(element: T) -> Box<Node<T>> {
        Box::new(Node { element, left: None, right: None })
    }
}

struct BinarySearchTree<T> {
    root: TreeLink<T>
}

impl<T: Ord> BinarySearchTree<T> {
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn insert(&mut self, element: T) {
        let mut copy_root = &mut self.root;

        while let Some(ref mut node) = copy_root {
            if node.element < element {
                copy_root = &mut node.right;
            } else {
                copy_root = &mut node.left;
            }
        }

        *copy_root = Some(Node::new(element));
    }

    fn search(&self, value: T) -> bool {
        let mut copy_root = &self.root;

        while let Some(ref node) = copy_root {
            if node.element == value {
                return true;
            } else if node.element < value {
                copy_root = &node.right;
            } else {
                copy_root = &node.left;
            }
        }

        return false;
    }

    fn inorder(&self) -> Vec<&T> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut current = &self.root;

        while let Some(node) = current {
            stack.push(node);
            current = &node.left;
        }

        while let Some(node) = stack.pop() {
            result.push(&node.element);
            current = &node.right;
            while let Some(node) = current {
                stack.push(node);
                current = &node.left;
            }
        }

        result
    }
}

#[cfg(test)]
mod test_bst {
    use super::*;

    #[test]
    fn test_insert() {
        let mut bst = BinarySearchTree::new();
        bst.insert(8);
        bst.insert(3);
        bst.insert(10);
        bst.insert(1);
        bst.insert(6);
        bst.insert(14);
        bst.insert(4);
        bst.insert(7);
        bst.insert(13);

        assert_eq!(bst.inorder(), vec![&1, &3, &4, &6, &7, &8, &10, &13, &14]);

    }

    #[test]
    fn test_search() {
        let mut bst = BinarySearchTree::new();
        bst.insert(8);
        bst.insert(3);
        bst.insert(10);
        bst.insert(1);
        bst.insert(6);
        bst.insert(14);
        bst.insert(4);
        bst.insert(7);
        bst.insert(13);
        assert_eq!(bst.search(4), true);
        assert_eq!(bst.search(16), false);
    }
        
}