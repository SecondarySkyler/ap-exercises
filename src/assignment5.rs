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
}


