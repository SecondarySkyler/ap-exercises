#![allow(dead_code)]
use core::fmt;
use std::{clone, result};

pub fn prev_str(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            if c == 'a' || c == 'A' || c == 'z' || c == 'Z' {
                result.push(c);
            } else {
                result.push((c as u8 - 1) as char);
            }
        } else {
            result.push(c);
        }
    }
    result
}

pub fn prev_char(c: char) -> char {
    if c.is_ascii_alphabetic() {
        if c == 'a' || c == 'A' || c == 'z' || c == 'Z' {
            c
        } else {
            (c as u8 - 1) as char
        }
    } else {
        c
    }
}

pub struct X {
    s: Option<String>,
    i: i32,
}

impl X {
    fn new(s: &str, n: i32) -> X {
        X {
            s: Some(s.to_string()),
            i: n,
        }
    }

    fn take_string(&mut self) -> Option<String> {
        self.s.take()
    }
}

pub struct NameSurname {
    name: String,
    surname: String,
}

// pub fn replace_surname(ns: NameSurname, new_surname: String) -> String {
//     std::mem::swap(&mut ns.surname, new_surname);
//     let result = String::from(&ns.surname);
//     result
// }

pub fn res1(n: i32) -> Result<i32, String> {
    if n % 10 == 0 {
        Ok(n)
    } else {
        Err("error".to_string())
    }
}

pub fn res2(r: Result<i32, &str>) -> Result<i32, String> {
    match r {
        Ok(n) => {
            if n % 5 == 0 {
                Ok(n)
            } else {
                Err("error".to_string())
            }
        }
        Err(_) => Err("error".to_string()),
    }
}

pub fn wrapper(n: i32) -> Result<i32, String> {
    let r1 = res1(n);
    let r2 = res2(r1.map_err(|_| "error"));

    r2
}

pub fn order(v: Vec<String>) -> Vec<String> {
    let mut result = vec![];

    v.iter().enumerate().for_each(|(i, s)| {
        result.push(format!("{} - {}", i, s));
    });

    result
} 


pub struct MaybePoint {
    pub x: Option<i32>,
    pub y: Option<i32>,
}

impl MaybePoint {
    pub fn new(x: Option<i32>, y: Option<i32>) -> MaybePoint {
        MaybePoint { x, y }
    }

    pub fn is_some(&self) -> bool {
        self.x.is_some() && self.y.is_some()
    }

    pub fn maybe_len(&self) -> Option<f32> {
        if self.is_some() {
            Some(((self.x.unwrap() * self.x.unwrap() + self.y.unwrap() * self.y.unwrap()) as f32).sqrt())
        } else {
            None
        }
    }
}

pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Size {
        Size { width, height }
    }

    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    pub fn compare(&self, other: &Size) -> Option<bool> {
        if self.area() == other.area() {
            None
        } else if self.area() > other.area() {
            Some(true)
        } else {
            Some(false)
        }
    }
}

#[derive(Clone,Debug)]
pub enum AirplaneCompany {
    Boeing,
    Airbus,
}
pub struct Airplane {
    pub company: AirplaneCompany,
    pub model: String,
}

pub struct AirFleet {
    pub fleet: Vec<Airplane>,
}

impl AirFleet {
    pub fn add_airplane(&mut self, airplane: Airplane) {
        self.fleet.push(airplane);
    }

    pub fn remove_boeing() {}

    pub fn search_airplane(&self, model: &str) -> Result<AirplaneCompany, String> {
        if self.fleet.is_empty() {
            return Err("Not in this fleet".to_string());
        }

        for airplane in &self.fleet {
            if airplane.model == model {
                return Ok(airplane.company.clone());
            }
        }

        Err("Not in this fleet".to_string())
    }
}

#[derive(Debug, Clone)]
pub struct Student {
    pub name: String,
    pub id: u32,
}

impl Student {
    pub fn new(name: &str, id: u32) -> Student {
        Student {
            name: name.to_string(),
            id,
        }
    }
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Student {{ name: \"{}\", id: \"{}\" }}", self.name, self.id)
    }
}

pub struct University {
    name: String,
    students: Vec<Student>,
}

impl University {
    pub fn new(name: &str, students: &[Student]) -> University {
        University {
            name: name.to_string(),
            students: students.to_vec(),
        }
    }

    pub fn remove_student(&mut self, id: u32) -> Result<Student, &str> {
        let mut index = None;
        for (i, student) in self.students.iter().enumerate() {
            if student.id == id {
                index = Some(i);
                break;
            }
        }

        match index {
            Some(i) => Ok(self.students.remove(i)),
            None => Err("Not found"),
        }
    }
}

impl fmt::Display for University {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", self.name)?;
        write!(f, "Students: [")?;
        for i in 0..self.students.len() - 1 {
            write!(f, "{}, ", self.students[i])?;
        }
        write!(f, "{}", self.students[self.students.len() - 1])?;
        write!(f, "]")?;
        Ok(())
    }
}