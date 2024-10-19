#![allow(dead_code)]

use std::{default, fmt::{Debug, Display}};

pub trait Printable {
    fn print(&self);
}

impl Printable for i32 {
    fn print(&self) {
       println!("{}", self); 
    }
}

impl Printable for String {
    fn print(&self) {
       println!("{}", self); 
    }
}

impl<T> Printable for Vec<T> where T: Printable {
    fn print(&self) {
        for item in self.iter() {
            item.print();
        }
    }
}

pub fn print<T: Printable>(g: T) {
    g.print();
}

// Easier way
// fn print(g: impl Printable) { g.print(); }

// Exercise 2
#[derive(Debug, Default)]
enum Category {
    Manga,
    #[default]
    Thriller,
    SciFi,
}

#[derive(Debug)]
pub struct Book<'a> {
    title: &'a str,
    cat: Category
}

#[derive(Debug, Default)]
pub struct Library<'a> {
    bookcases: [Vec<Book<'a>>; 10]
}

impl Default for Book<'_> {
   fn default() -> Self {
       Book { title: "default", cat: Category::default() }
   } 
}

impl Book<'_> {
    fn default_with_cat(c: Category) -> Self {
        Book {
            cat: c,
            ..Self::default()
        }
    }
}

trait Populatable {
    fn populate(&mut self);
}

impl Populatable for Library<'_> {
    fn populate(&mut self) {
        for floor in self.bookcases.iter_mut() {
            floor.push(Book::default());
            floor.push(Book::default());
            floor.push(Book::default());
        }
    }
}

// Exercise 3
pub fn restricted<T: PartialOrd + Debug, U: Display>(t1: T, t2: T, u: U) -> T {
    let min = if t1 < t2 { t1 } else { t2 };
    println!("minor: {:?}", min);
    println!("u: {}", u);
    min
}

// Exercise 4
struct Tasks {
    pub tasks: Vec<Task>,
}

#[derive(Clone, Debug)]
struct Task {
    name: String,
    priority: i32,
    done: bool
}

impl Task {
    fn new(name: String, priority: i32, done: bool) -> Self {
        Task { name, priority, done } // doesnt make sense...
    }
}

impl Tasks {
    fn new() -> Self {
        Tasks { tasks: Vec::new() }
    }

    fn insert_task(&mut self, nt: Task) {
        self.tasks.push(nt);
    }


    fn iter(&mut self) -> TaskIterator {
        self.tasks.retain(|task| !task.done);
        TaskIterator {
            list_tasks: self,
            it_index: 0
        }
    }
}


struct TaskIterator<'a> {
    list_tasks: &'a Tasks,
    it_index: usize
}

impl<'a> Iterator for TaskIterator<'a> {
    type Item = &'a Task;

    fn next(&mut self) -> Option<Self::Item> {
       if self.it_index < self.list_tasks.tasks.len() {
           let task = Some(&self.list_tasks.tasks[self.it_index]);
           self.it_index += 1;
           task
       } else {
           None
       } 
        
    }
}


#[cfg(test)]
mod test5 {
    use super::*;
    
    #[test]
    fn test_ex2() {
        let mut lib = Library::default();
        lib.populate();
        for f in lib.bookcases.iter() {
            assert_eq!(f.len(), 3);
        }
    }

    #[test]
    fn test_ex3() {
        assert_eq!(restricted(1, 2, 3), 1);
    }

    #[test]
    fn test_ex4() {
        let  t1: Task = Task::new("t1".to_string(), 1, true);
        let  t2: Task = Task::new("t2".to_string(), 1, false);
        let mut tm: Tasks = Tasks::new();
        tm.insert_task(t1);
        tm.insert_task(t2);

        for task in tm.iter() {
            println!("{:?}", task);
        }
    }
}


