#![allow(dead_code)]

use std::{default, fmt::{Debug, Display}};
use std::ops::{Add, Sub, Mul};
use rand::{self, Rng};

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

// Exercise 5
#[derive(Debug, PartialEq, Eq)]
struct Pair(i32, String);

impl Add<i32> for Pair {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
      Pair(self.0 + rhs, self.1)  
    }
}

impl Sub<i32> for Pair {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
      Pair(self.0 - rhs, self.1)  
    }
}

impl Add<&str> for Pair {
    type Output = Self;

    fn add(self, rhs: &str) -> Self::Output {
        Pair(self.0, self.1 + rhs)
    }
}

impl Sub<&str> for Pair {
    type Output = Self;

    fn sub(self, rhs: &str) -> Self::Output {
        Pair(self.0, self.1.replace(rhs, ""))
    }
}

impl Add for Pair {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self + rhs.0 + rhs.1.as_str()
    }
}

impl Sub for Pair {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self - rhs.0 - rhs.1.as_str()
    }
}

impl Mul<i32> for Pair {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Pair(self.0.pow(rhs as u32), self.1.repeat(rhs as usize))
    }
}

// Exercise 6
#[derive(Debug)]
struct Open;

#[derive(Debug)]
struct Closed;

#[derive(Debug)]
struct Stopped {
    reason: String
}

struct Gate<S> {
    state: S
}

impl Gate<Open> {
    fn new() -> Gate<Open> {
        Gate { state: Open }
    }

    fn close(self) -> Result<Gate<Closed>, Gate<Stopped>> {
        let r = rand::thread_rng().gen_range(0..10);
        if r <= 7 {
            return Ok(Gate { state: Closed })
        } else {
            return Err(Gate { state: Stopped { reason: "Bo".to_string() } })
        }
    }
}

impl Gate<Closed> {
    fn new() -> Gate<Closed> {
        Gate { state: Closed }
    }

    fn open(self) ->Result<Gate<Open>, Gate<Stopped>> {
        let r = rand::thread_rng().gen_range(0..10);
        if r <= 7 {
            return Ok(Gate { state: Open })
        } else {
            return Err(Gate { state: Stopped { reason: "Bo".to_string() } })
        }
    }
}

impl Gate<Stopped> {
    fn new(reason: String) -> Gate<Stopped> {
        Gate { state: Stopped { reason } }
    }

    fn open(self) -> Gate<Open> {
        Gate { state: Open }
    }

    fn close(self) -> Gate<Closed> {
        Gate { state: Closed }
    }
}

// Exercise 7
trait Heatable {
    fn cook(&mut self);
}

trait Friable {
    fn cook(&mut self);
}

trait Heater {
    fn heat(&self, heatable: &mut dyn Heatable);
}

trait Frier {
    fn fry(&self, friable: &mut dyn Friable);
}

struct Oven;
struct Pan;

impl Heater for Oven {
    fn heat(&self, heatable: &mut dyn Heatable) {
        heatable.cook();
    }
}

impl Heater for Pan {
    fn heat(&self, heatable: &mut dyn Heatable) {
        heatable.cook();
    }
}

impl Frier for Pan {
    fn fry(&self, friable: &mut dyn Friable) {
        friable.cook();
    }
}

struct Pie {
    ready: bool
}

#[derive(PartialEq, Eq)]
enum CarrotState {
    Raw,
    Cooked,
    Fried,
    Burnt
}

struct Carrot {
    state: CarrotState
}

trait Edible {
    fn eat(&self);
}

impl Heatable for Pie {
    fn cook(&mut self) {
        match self.ready {
            true => println!("you burned the pie!!"),
            false => { self.ready = true; }
        }
    }
}

impl Heatable for Carrot {
    fn cook(&mut self) {
        match self.state {
            CarrotState::Raw => { self.state = CarrotState::Cooked; },
            _ => { self.state = CarrotState::Burnt; }
        }
    }
}

impl Friable for Carrot {
    fn cook(&mut self) {
        /* if self.state == CarrotState::Fried {
            self.state = CarrotState::Burnt;
        } else {
            self.state = CarrotState::Fried;
        } */
        match self.state {
            CarrotState::Fried => { self.state = CarrotState::Burnt; },
            _ => { self.state = CarrotState::Fried; }
        }
    }
}

impl Edible for Pie {
    fn eat(&self) {
        match self.ready {
            true => println!("yummy"),
            false => println!("you got stomach ache"),
        }
    }
}

impl Edible for Carrot {
    fn eat(&self) {
        match self.state {
            CarrotState::Raw => println!("mmh, crunchy"),
            CarrotState::Cooked => println!("mmh, yummy"),
            CarrotState::Fried => println!("mmh, crispy"),
            CarrotState::Burnt => println!("mmh, burnt"),
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

    #[test]
    fn test_ex5() {
        let p1 = Pair(6, "hello".to_string());
        let p2 = Pair(3, "hello".to_string());
        assert_eq!(p1 + 4, Pair(10, "hello".to_string()));
        assert_eq!(p2 + " world", Pair(3, "hello world".to_string()));
    }
}
