#![allow(unused)]
use core::{f32, fmt};
use std::{collections::LinkedList, ops::{Add, Sub}, result};

pub fn find_equal<'a>(s1: &'a str, s2: &'a str) -> Option<(&'a str, &'a str)> {
    for i in 0..s1.len() - 1{
        let slice1 = &s1[i..i + 2]; 

        if let Some(j) = s2.find(slice1) {
            return Some((slice1, &s2[j..j + 2]));
        }
    }
    None
}

// fn random_letter() -> char{
//     let mut n = rand::random::<u8>();
//     n = n % 26 + 'a' as u8;
//     n as char
// }
// fn random_string(len: usize) -> String{
//     let mut s = String::with_capacity(len);
//     for _ in 0 .len{
//         s.push(random_letter());
//     }
//     s
// }

// pub fn lucky_slice(input_str: &str) -> Option<&str> {
//     let second_str = random_string(input_str.len());
//     let result = find_equal(input_str, &second_str);
//     if result.is_none(){
//         return None;
//     }
//     return Some(result.unwrap().0);
// }

// Exercise 2
pub struct Person<'a> {
    pub name: String,
    pub father: Option<&'a Person<'a>>,
    pub mother: Option<&'a Person<'a>>
}

impl<'a> Person<'a> {
    pub fn new(name: String, father: Option<&'a Person<'a>>, mother: Option<&'a Person<'a>>) -> Self {
        Self {name, father, mother}
    }

    fn rec_find_relative<'b> (&'b self, generations: u32, relatives: &mut Vec<&'b Self>) {
        relatives.push(self);

        if generations > 0 {
            if let Some(mother) = self.mother {
                mother.rec_find_relative(generations - 1, relatives);
            }

            if let Some(father) = self.father {
                father.rec_find_relative(generations - 1, relatives);
            }
        }
    }
    pub fn find_relatives(&self, generations: u32) -> Vec<&Person> {
        let mut relatives = Vec::new();
        self.rec_find_relative(generations, &mut relatives);
        relatives
    }

    fn rec_find_roots<'b>(&'b self, roots: &mut Vec<&'b Self>) {
        match (self.mother, self.father) {
            (None, None) => roots.push(self),
            (None, Some(f)) => {
                f.rec_find_roots(roots);
                roots.push(self);
            },
            (Some(m), None) => {
                m.rec_find_roots(roots);
                roots.push(self);
            },
            (Some(m), Some(f)) => {
                m.rec_find_roots(roots);
                f.rec_find_roots(roots);
            },
        }
    }
    pub fn find_roots(&self) -> Vec<&Person> {
        let mut roots = Vec::new();
        self.rec_find_roots(&mut roots);
        roots
    }
}

// Exercise 3 
struct ImportantExcerpt<'a> {
    part: &'a str
}

impl <'a: 'b, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please {}", announcement);
        self.part
    }
}

// Exercise 4
struct DoubleRef<'a: 'b, 'b, T> {
    r: &'a T,
    s : &'b T
}

// Exercise 5
trait Split<'a> {
    type ReturnType;
    fn split(&'a self) -> (Self::ReturnType, Self::ReturnType);
}

impl<'a> Split<'a> for String {
    type ReturnType = &'a str;
    fn split(&'a self) -> (Self::ReturnType, Self::ReturnType) {
        (&self[0..self.len()/2], &self[self.len()/2+1..])
        // self.as_str().split_at(self.len() / 2)
    }
}

impl<'a> Split<'a> for &[i32] {
    type ReturnType = &'a [i32];

    fn split(&'a self) -> (Self::ReturnType, Self::ReturnType) {
        (&self[0..self.len()/2], &self[self.len() / 2 + 1..])
        // self.split_at(self.len() / 2)
    }
}

impl<'a> Split<'a> for LinkedList<f64> {
    type ReturnType = LinkedList<f64>;

    fn split(&'a self) -> (Self::ReturnType, Self::ReturnType) {
        let mut left = self.clone();
        let right = left.split_off(left.len() / 2);
        (left, right)
    }
}

// Exercise 6
pub struct Point {
    pub x: f32,
    pub y: f32
}

pub struct Circle {
    radius: f32,
    center: Point
}
pub struct Rectangle {
    top_left: Point,
    bottom_right: Point
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 0., y: 0. }
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self { radius: 1.0, center: Point::default() }
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self { top_left: Point { x: -1., y: 1. }, bottom_right: Point { x: 1., y: -1. }}
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Area {
    pub area: f32
}

impl Default for Area {
    fn default() -> Self {
        Self { area: 0. }
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Area is {} cm2", self.area)
    }
}

pub trait GetArea {
    fn get_area(&self) -> Area;
}

impl GetArea for Point {
    fn get_area(&self) -> Area {
        Area { area: 0.0 }
    }
}

impl GetArea for Circle {
    fn get_area(&self) -> Area {
        Area {
            area: f32::consts::PI * self.radius.powi(2)
        }
        
    }
}

impl GetArea for Rectangle {
    fn get_area(&self) -> Area {
        let height = self.bottom_right.y - self.top_left.y;
        let width = self.bottom_right.x - self.top_left.x;
        Area { area: width * height }
    }
}

impl Add for Area {
    type Output = Area;

    fn add(self, rhs: Self) -> Self::Output {
        Area {
            area: self.area + rhs.area
        }
    }
}

pub fn sum_area(items: &[&dyn GetArea]) -> Area {
    let mut res = Area::default();
    items.iter().for_each(|shape| res = res + shape.get_area());
    res
}

// Ex7
pub fn skip_prefix<'a>(telephone_number: &'a str, prefix: &'a str) -> &'a str {
    if telephone_number.contains(prefix) {
        let mut tel_clone = telephone_number;
        tel_clone.replace(prefix, "");
        return tel_clone
    } else {
        return telephone_number
    }
}


#[cfg(test)]
mod test4 {
    use super::*;

    #[test]
    fn test_find_equal() {
        assert_eq!(find_equal("hello", "world"), None);
        assert_eq!(find_equal("hello", "hell"), Some(("he", "he")));
    }
}