#![allow(dead_code)]

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

#[cfg(test)]
mod test3 {
    use super::*;

    #[test]
    fn test_is_it_luhn() {
        assert_eq!(is_it_luhn("4539 3195 0343 6467".to_string()), true);
    }
}