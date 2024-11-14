#![allow(unused)]

use std::{cell::RefCell, collections::{HashSet, LinkedList, VecDeque}, fmt::Debug, hash::Hash, rc::Rc};

// Exercise 1
mod odd_module {
    pub const CONSTANT: i32 = 123;
}

mod even_module {
    pub const CONSTANT: i32 = 246;
}

mod getter_function {
    use super::{even_module, odd_module};

    pub fn get_constant(value: u32) -> i32 {
        if value % 2 == 0 {
            even_module::CONSTANT
        } else {
            odd_module::CONSTANT
        }
    }
}

// Exercise 2
trait CloneAndDouble {
    fn clone_and_double(&self) -> Self;
}

impl<T: Clone + std::ops::Add<Output = T>> CloneAndDouble for T {
    fn clone_and_double(&self) -> Self {
        self.clone() + self.clone()
    }
}

// Exercise 3 
trait Unknown {
    fn serialize(&self) -> String;
}

impl Unknown for i32 {
    fn serialize(&self) -> String {
        self.to_string()
    }
}

impl Unknown for String {
    fn serialize(&self) -> String {
        self.clone()
    }
}

impl<T: Debug> Unknown for Vec<T> {
    fn serialize(&self) -> String {
        format!("{:?}", self)
    }
}

fn get_vec() -> Vec<Box<dyn Unknown>> {
    Vec::new()
}

fn print_vec(v: &Vec<Box<dyn Unknown>>) {
    for el in v {
        println!("{}", el.serialize())
    }
}

// Exercise 4
struct BinIter {
    n: u128,
    l: usize
}

impl BinIter {
    fn new(n: u128, l: usize) -> Self {
        BinIter { n, l }
    }
}

impl Iterator for BinIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let output = Some(self.n % 2 != 0);
        self.n >>= 1;
        if self.l == 0 {
            None
        } else {
            self.l -= 1;
            output
        }
    }
}

// Exercise 5

// Exercise 6
type NodeRef<T> = Rc<Node<T>>;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Node<T: Hash + PartialEq + Eq> {
    element: T,
    neighbours: Vec<NodeRef<T>>
}

impl<T: Hash + PartialEq + Eq> Node<T> {
    fn new(element: T, neighbours: Vec<NodeRef<T>>) -> Self {
        Node { element, neighbours }
    }

    fn get_value(&self) -> &T {
        &self.element
    }
}

impl<T: Hash + PartialEq + Eq + Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let neighbours = self.neighbours
        .iter()
        .map(|node| format!("{:?}", node.get_value()))
        .collect::<Vec<_>>();
        let neighbours_str = format!("[{}]", neighbours.join(", "));
        write!(f, "[value: {:?}, adjacents: {:?}]", self.element, neighbours_str)
    }
}

struct Graph<T: Hash + PartialEq + Eq> {
    nodes: Vec<NodeRef<T>>
}

impl<T: Hash + PartialEq + Eq> Graph<T> {
    fn new(nodes: Vec<NodeRef<T>>) -> Self {
        Graph { nodes }
    }

    fn dfs(&self, root: NodeRef<T>) -> Vec<NodeRef<T>> {
        let mut history = Vec::<NodeRef<T>>::new();
        let mut visited = HashSet::<NodeRef<T>>::new();
        let mut stack = VecDeque::<NodeRef<T>>::new();
        stack.push_back(root);

        while let Some(node) = stack.pop_front()  {
            if visited.insert(Rc::clone(&node)) {
                history.push(Rc::clone(&node));
                for neighbor in node.neighbours.iter().rev() {
                    stack.push_front(Rc::clone(&neighbor));
                }

            }
        }

        history
    }
}

// Exercise 7
trait Task {
    fn execute(&self) -> usize;
}

struct SumTask {
    n1: usize,
    n2: usize
}

impl SumTask {
    fn new(n1: usize, n2: usize) -> Self {
        SumTask { n1, n2 }
    }
}

impl Task for SumTask {
    fn execute(&self) -> usize {
        self.n1 + self.n2
    }
}

struct LenTask {
    s: String
}

impl LenTask {
    fn new(s: String) -> Self {
        LenTask { s }
    }
}

impl Task for LenTask {
    fn execute(&self) -> usize {
        self.s.len()
    }
}

struct TaskQueue {
    queue: LinkedList<Box<dyn Task>>
}

impl TaskQueue {
    fn new() -> Self {
        TaskQueue { queue: LinkedList::new() }
    }

    fn push(&mut self, task: Box<dyn Task>) {
        self.queue.push_back(task);
    }

    fn pop(&mut self) -> Option<Box<dyn Task>> {
        self.queue.pop_front()
    }
}

struct Tasker {
    queue: Rc<RefCell<TaskQueue>>
}

impl Tasker {
    fn new() -> Self {
        Tasker { queue: Rc::new(RefCell::new(TaskQueue::new())) }
    }

    fn get_tasker(&self) -> Tasker {
        Tasker { queue: Rc::clone(&self.queue) }
    }

    fn get_executer(&self) -> Executer {
        Executer { queue: Rc::clone(&self.queue) }
    }

    fn schedule_task(&mut self, task: Box<dyn Task>) {
        self.queue.borrow_mut().push(task);
    }
}

struct Executer {
    queue: Rc<RefCell<TaskQueue>>
}

impl Executer {
    fn execute_task(&mut self) -> Option<usize> {
        match self.queue.borrow_mut().pop() {
            Some(task) => Some(task.execute()),
            None => None,
        }
    }
}

#[cfg(test)]
mod test_mt2 {
    use super::*;
    #[test]
    fn test_getter_func() {
        assert_eq!(getter_function::get_constant(2), even_module::CONSTANT);
        assert_eq!(getter_function::get_constant(3), odd_module::CONSTANT);
    }

    #[test]
    fn clone_double() {
        assert_eq!(2.clone_and_double(), 4);
    }

    #[test]
    fn test_unknown() {
        let mut v = get_vec();
        v.push(Box::new(32));
        v.push(Box::new("value".to_string()));
        print_vec(&v);
    }

    #[test]
    fn test_graph() {
        let node3 = Rc::new(Node::new(3, vec![]));
        let node4 = Rc::new(Node::new(4, vec![]));
        let node5 = Rc::new(Node::new(5, vec![]));
        let node6 = Rc::new(Node::new(6, vec![]));

        let node1 = Rc::new(Node::new(1, vec![Rc::clone(&node4), Rc::clone(&node5)]));
        let node2 = Rc::new(Node::new(2, vec![Rc::clone(&node6)]));

        let node0 = Rc::new(Node::new(0, vec![Rc::clone(&node1), Rc::clone(&node2), Rc::clone(&node3)]));

        let graph = Graph::new(vec![
            Rc::clone(&node0),
            Rc::clone(&node1), Rc::clone(&node2),Rc::clone(&node3),
            Rc::clone(&node4), Rc::clone(&node5),Rc::clone(&node6),
        ]);

        let visit = graph.dfs(Rc::clone(&node0));
        for node in visit.iter() {
            print!("{:?}", node.get_value());
        }
    }

    #[test]
    fn tasker() {
        let mut tasker = Tasker::new();
        let sum_task = SumTask::new(3, 5);
        let len_task = LenTask::new("Hello".to_string());

        tasker.schedule_task(Box::new(sum_task));
        tasker.schedule_task(Box::new(len_task));

        let mut executer = tasker.get_executer();

        assert_eq!(executer.execute_task(), Some(8));
        assert_eq!(executer.execute_task(), Some(5));
        assert_eq!(executer.execute_task(), None);
    }
}