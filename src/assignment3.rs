#![allow(dead_code)]

use core::{fmt, str};
use std::collections::HashMap;
// Exercise 1: Given a number determine whether it is valid per the Luhn formula
pub fn is_it_luhn(s: String) -> bool {
    if s.len() <= 1 {
        return false;
    }

    let clone = s.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    
    if clone.chars().any(|c| !c.is_digit(10)) {
        return false;
    }

    clone.chars().rev().enumerate().fold(0, |acc, (i, c)| {
        let mut digit = c.to_digit(10).unwrap();
        if i % 2 == 1 {
            digit *= 2;
            if digit > 9 {
                digit -= 9;
            }
        }
        acc + digit
    }) % 10 == 0

    
}

// Exercise 2: For the following examples, decide which of the composite data structures is better (enum or structs). Then implement them
enum Fuel {
    Diesel,
    Gasoline,
    LPG,
    Methane,
    Electric
}

enum IpFamily {
    V4(i32, i32, i32, i32),
    V6(i16, i16, i16, i16, i16, i16, i16, i16)
}

struct Point {
    x: f64,
    y: f64,
    z: f64
}

// Exercise 3: Associate the number plate with the car owner
pub struct Car {
    pub owner: String,
    pub number_plate: String
}

pub fn recognise_owner(price_tracker: &HashMap<Car, f32>, number_plate: String) -> Option<String> {
    for (car, _) in price_tracker {
        if car.number_plate == number_plate {
            return Some(car.owner.clone());
        }
    }
    None
}

// Exercise 4: Create a vending machine
#[derive(PartialEq, Eq, Hash)]
enum Item {
    Coke,
    Pepsi,
    Coffee
}

enum Coin {
    Cent5,
    Cent10,
    Cent20,
    Cent50,
    Euro1,
    Euro2
}

impl Coin {
    fn to_cents(&self) -> i32 {
        match self {
            Coin::Cent5 => 5,
            Coin::Cent10 => 10,
            Coin::Cent20 => 20,
            Coin::Cent50 => 50,
            Coin::Euro1 => 100,
            Coin::Euro2 => 200
        }
    }
}

struct VendingMachine {
    items: HashMap<Item, usize>,
    coins: u32
}

impl VendingMachine {
    fn new(items: HashMap<Item, usize>) -> VendingMachine {
        VendingMachine {
            items,
            coins: 0
        }
    }

    fn add_item(&mut self, item: Item, quantity: usize) {
        self.items.insert(item, quantity);
    }

    fn insert_coin(&mut self, coin: Coin) -> Result<&str, &str> {
        let conv = coin.to_cents();
        if conv == 0 {
            return Err("Invalid coin");
        }
        self.coins += conv as u32;
        Ok("Coin inserted")
    }

    fn get_item_price(&self, item: &Item) -> u32 {
        match item {
            Item::Coke => 100,
            Item::Pepsi => 120,
            Item::Coffee => 150
        }
    }

    fn buy(&mut self, item: Item) -> Result<u32, &str> {
        if self.items.contains_key(&item) {
            let price = self.get_item_price(&item);
            if price <= self.coins {
                self.coins -= price;
                let quantity = self.items.get_mut(&item).unwrap();
                if *quantity > 0 {
                    *quantity -= 1;
                    return Ok(price);
                }
            }
        }
        Err("Not enough money or item out of stock")
    }
}

// Exercise 5: Implement two tuple structs named Date and Hour
#[derive(Debug)]
struct Date {
    day: u8,
    month: u8,
    year: u16
}

#[derive(Debug)]
struct Hour {
    hour: u8,
    minute: u8,
}

#[derive(Debug)]
struct BoxShipping {
    name: String,
    barcode: String,
    shipment_date: Date,
    shipment_hour: Hour
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}/{:02}/{:04}", self.day, self.month, self.year)
    }
}

impl fmt::Display for Hour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}

impl fmt::Display for BoxShipping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}\nBarcode: {}\nShipment Date: {}\nShipment Hour: {}", self.name, self.barcode, self.shipment_date, self.shipment_hour)
    }
}

// Exercise 6: Create BUP's library system. It should be able to store books, articles and magazines.
struct Book {
    name: String,
    code: u32,
    year_of_publication: u16,
    author: String,
    publisher: String
}

struct Article {
    name: String,
    code: u32,
    year_of_publication: u16,
    orchid: String
}

struct Magazine {
    name: String,
    code: u32,
    year_of_publication: u16,
    number: u32,
    month: u8
}

struct Bup {
    books: Vec<Book>,
    articles: Vec<Article>,
    magazines: Vec<Magazine>
}

// maybe there is a better way to do this, avoiding three vectors
// also I won't implement the Display trait :§
impl Bup {
    fn new() -> Bup {
        Bup {
            books: Vec::new(),
            articles: Vec::new(),
            magazines: Vec::new()
        }
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn add_article(&mut self, article: Article) {
        self.articles.push(article);
    }

    fn add_magazine(&mut self, magazine: Magazine) {
        self.magazines.push(magazine);
    }

    
}

// Exercise 7: Create a module called Point that inside has a struct Point with the fields x: f32 , y: f32
pub mod point {
    pub struct Point {
        pub x: f32,
        pub y: f32
    }

    impl Point {
        pub fn new(x: f32, y: f32) -> Point {
            Point {
                x,
                y
            }
        }

        pub fn distance(&self, other: &Point) -> f32 {
            ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
        }
    }
}

pub mod line {
    use std::f32::consts::E;

    use super::point::Point;

    pub struct Line {
        pub start: Point,
        pub end: Point,
        pub m: f32,
        pub q: f32
    }

    impl Line {
        pub fn new(start: Point, end: Point) -> Line {
            let m = (end.y - start.y) / (end.x - start.x);
            let q = start.y - m * start.x;
            Line {
                start,
                end,
                m,
                q
            }
        }

        pub fn contains(&self, point: &Point) -> Result<(), String> {
            if (self.m * point.x + self.q) == point.y {
                Ok(())
            } else {
                Err("Point is not in the line".to_string())
            }
        }
        
    }
}

pub mod sentence {
    pub struct Sentence {
        pub words: Vec<String>
    }

    impl Sentence {
        pub fn new_default() -> Sentence {
            Sentence {
                words: Vec::new()
            }
        }

        pub fn new(s: &str) -> Sentence {
            Sentence {
                words: s.split_whitespace().map(|s| s.to_string()).collect()
            }
        }
    }
}

mod magicmodule {
    use std::collections::HashMap;

    use super::sentence::Sentence;

    pub fn magic_sentence(hm: &mut HashMap<i32, Sentence>, i: i32, j: i32) -> Result<Sentence, &str> {
        let o1 = hm.get(&i);
        let o2 = hm.get(&j);

        if o1.is_none() && o2.is_none() {
            return Err("Both sentences are not in the hashmap");
        }

        let s1 = &o1.unwrap().words;
        let s2 = &o2.unwrap().words;

        Ok(Sentence {
            words: s1.iter().zip(s2.iter()).filter_map(|(w1, w2)| {
                if w1 == w2 {
                    Some(w1.clone())
                } else {
                    None
                }
            }).collect()
        })

    
    }
}

#[cfg(test)]
mod test3 {
    use super::*;

    #[test]
    fn test_is_it_luhn() {
        assert_eq!(is_it_luhn("4539 3195 0343 6467".to_string()), true);
    }

    #[test]
    fn test_contains() {
        let p1 = point::Point::new(1.0, 1.0);
        let p2 = point::Point::new(2.0, 2.0);
        let p3 = point::Point::new(3.0, 3.0);
        let l = line::Line::new(p1, p2);
        assert_eq!(l.contains(&p3), Ok(()));

    }
}