use crate::assignment5::print;

// Exercise 1
struct TreeNode<T> where T: PartialOrd + Clone {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>
}

impl<T> TreeNode<T> where T: PartialOrd + Clone {
    fn new(value: T) -> Self {
        TreeNode { value, left: None, right: None }
    }

    fn from_vec(mut values: Vec<T>) -> Self {
        let first_item = values.remove(0);
        let mut tree = TreeNode::new(first_item);

        for item in values {
            tree.insert(item);
        }

        tree
    }

    fn insert(&mut self, value: T) {
        let target_node = if value < self.value {
            &mut self.left
        } else {
            &mut self.right
        };
        match target_node {
            Some(ref mut node) => {
                node.insert(value);
            }
            None => {
                let node = TreeNode::new(value);
                *target_node = Some(Box::new(node));
            }
        }
    }
}
// Exercise 2
#[derive(Debug)]
struct Car {
    model: String,
    year: u32,
    price: u32,
    rent: bool
}

struct CarDealer {
    cars: Vec<Car>
}

struct User {
    owned_car: Option<Car>
}

impl Car {
    fn new(model: String, year: u32, price: u32) -> Self {
        Car { model, year, price, rent: false }
    }
}

impl Default for Car {
    fn default() -> Self {
        Car { model: "Volvo".to_string(), year: 2000, price: 20, rent: false }
    }
}

impl CarDealer {
    fn new(cars: Vec<Car>) -> Self {
        CarDealer { cars }
    }

    fn add_car(&mut self, car: Car) {
        self.cars.push(car);
    }

    fn rent_user(&mut self, user: &mut User, model: String) {
        if let Some(pos) = self.cars.iter().position(|car| car.model == model) {
            user.owned_car = self.cars.get(pos);
        } else {
            println!("Car not found");
        }
    }
}


impl User {
    fn print_car(&self) {
        match &self.owned_car {
            Some(car) => println!("{:?}", car),
            None => println!("User has no car")
        }
    }
}
