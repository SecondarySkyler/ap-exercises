#![allow(unused)]

use std::{cell::{Ref, RefCell}, fmt::Debug, rc::Rc, marker::PhantomData};

// Exercise 1
struct TreeNode<T> where T: PartialOrd + Clone + Debug {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>
}

impl<T> TreeNode<T> where T: PartialOrd + Clone + Debug {
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

    fn preorder(&self) {
        println!("{:?}", self.value);
        if let Some(ref left) = self.left {
            left.preorder();
        }

        if let Some(ref right) = self.right {
            right.preorder();
        }
    }

    fn inorder(&self) {
        if let Some(ref left) = self.left {
            left.preorder();
        }
        println!("{:?}", self.value);

        if let Some(ref right) = self.right {
            right.preorder();
        }
    }

    fn postorder(&self) {
        if let Some(ref left) = self.left {
            left.preorder();
        }
        
        if let Some(ref right) = self.right {
            right.preorder();
        }
        println!("{:?}", self.value);
    }

}

// Exercise 2
type CarRef = Rc<RefCell<Car>>;

#[derive(Debug)]
struct Car {
    model: String,
    year: u32,
    price: u32,
    rent: bool
}

struct CarDealer {
    cars: Vec<CarRef>
}

struct User {
    pub owned_car: Option<CarRef>
}

impl Car {
    fn new(model: String, year: u32, price: u32) -> Self {
        Car { model, year, price, rent: false }
    }
}

impl Default for Car {
    fn default() -> Self {
        Car { model: "".to_string(), year: 0, price: 0, rent: false }
    }
}

impl CarDealer {
    fn new(cars: Vec<CarRef>) -> Self {
        CarDealer { cars }
    }

    fn add_car(&mut self, car: Car) {
        self.cars.push(Rc::new(RefCell::new(car)));
    }

    fn rent_user(&mut self, user: &mut User, model: String) {
        if let Some(pos) = self.cars.iter().position(|car| car.borrow().model == model) {
            let rented_car = Rc::clone(&self.cars[pos].clone());
            rented_car.borrow_mut().rent = true;
            user.owned_car = Some(rented_car);
        } else {
            println!("Car not found");
        }
    }

    fn end_rental(&mut self, user: &mut User) {
        match user.owned_car.take() {
            Some(rented_car) => {
                rented_car.borrow_mut().rent = false;
            },
            None => {
                println!("User has no car")
            },
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


// Exercise 3
trait Sound {
    fn make_sound(&self) -> String;
}

struct Cow;
impl Sound for Cow {
    fn make_sound(&self) -> String {
        "Moo".to_string()
    }
}

struct Dog;
impl Sound for Dog {
    fn make_sound(&self) -> String {
        "Bau".to_string()
    }
}

struct Farmcell {
    element: Box<dyn Sound>,
    next: Option<Box<Farmcell>>
}

impl Farmcell {
    fn new(el: Box<dyn Sound>) -> Self {
        Farmcell { element: el, next: None }
    }

    fn insert(&mut self, new_el: Box<dyn Sound>) {
        match self.next {
            Some(ref mut next) => next.insert(new_el),
            None => self.next = Some(Box::new(Farmcell::new(new_el)))
        }
    }
}

impl Sound for Farmcell {
    fn make_sound(&self) -> String {
        let mut result = self.element.make_sound();

        if let Some(ref next) = self.next {
            result.push_str(next.make_sound().as_str());
        }
        result
    }
}

// Exercise 4
#[derive(Debug, Clone)]
struct PublicStreetLight {
    id: u32,
    on: bool,
    burn_out: bool
}

impl PublicStreetLight {
    fn new(id: u32, on: bool, burn_out: bool) -> Self {
        PublicStreetLight { id, on, burn_out }
    }
}

impl Default for PublicStreetLight {
    fn default() -> Self {
        PublicStreetLight { id: 0, on: false, burn_out: false }
    }
}

struct PublicIllumination {
    lights: Vec<PublicStreetLight>
}

impl PublicIllumination {
    fn new(lights: Vec<PublicStreetLight>) -> Self {
        Self { lights }
    }

    // could be a good implementation but...
    // fn iter_mut(&mut self) -> PublicIlluminationIterator {
    //     self.lights.retain(|light| !light.burn_out);
    //     PublicIlluminationIterator {
    //         container: self,
    //         index: 0
    //     }
    // }
}

impl Default for PublicIllumination {
    fn default() -> Self {
        PublicIllumination { lights: vec![PublicStreetLight::default()] }
    }
}

impl Iterator for PublicIllumination {
    type Item = PublicStreetLight;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pos) = self.lights.iter().position(|light| light.burn_out) {
            return Some(self.lights.remove(pos))
        }
        None
    }
}

// Haters will see this and start crying
// struct PublicIlluminationIterator<'a> {
//     container: &'a PublicIllumination,
//     index: usize
// }


// impl<'a> Iterator for PublicIlluminationIterator<'a> {
//     type Item = &'a PublicStreetLight;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.index < self.container.lights.len() {
//             let broken_light = Some(&self.container.lights[self.index]);
//             self.index += 1;
//             broken_light
//         } else {
//             return None
//         }
//     }
// }


// Exercise 5
trait CompileTimeNode {
    type LeftType: CompileTimeNode;
    type RightType: CompileTimeNode;
    fn is_none() -> bool;
}

struct NullNode {}

impl CompileTimeNode for NullNode {
    type LeftType = NullNode;
    type RightType = NullNode;

    fn is_none() -> bool {
        true
    }
}

struct Node<L: CompileTimeNode, R: CompileTimeNode> {
    left: PhantomData<L>,
    right: PhantomData<R>
}

impl<L: CompileTimeNode, R: CompileTimeNode> CompileTimeNode for Node<L, R> {
    type LeftType = L;
    type RightType = R;

    fn is_none() -> bool {
        false
    }
}

fn count_nodes<T: CompileTimeNode>() -> usize {
    let mut count: usize = 0;
    if !T::is_none() {
        count = 1;
        count += count_nodes::<T::LeftType>();
        count += count_nodes::<T::RightType>();
    }
    count
}

// Exercise 6
struct EntangledBit {
    bit: Rc<RefCell<bool>>
}

impl Default for EntangledBit {
    fn default() -> Self {
        Self { bit: Rc::new(RefCell::new(false)) }
    }
}

impl EntangledBit {
    fn set(&mut self) {
        *self.bit.borrow_mut() = true;
    }

    fn reset(&mut self) {
        *self.bit.borrow_mut() = false;
    }

    fn get(&self) -> bool {
        *self.bit.borrow()
    }

    fn entangle_with(&self, other: &mut Self) {
        other.bit = self.bit.clone();
    }
}

#[cfg(test)]
mod test6 {
    use super::*;

    #[test]
    fn test_ex1() {
        let vn = vec![10, 5, 20];
        let tree = TreeNode::from_vec(vn);
        // tree.preorder();
        // tree.inorder();
        // tree.postorder();
    }

    #[test]
    fn test_ex2() {
        let audi = Car::new("Audi".to_string(), 2024, 60_000);
        let bmw = Car::new("BMW".to_string(), 2024, 60_000);

        let mut car_dealer = CarDealer::new(vec![
            Rc::new(RefCell::new(audi)),
            Rc::new(RefCell::new(bmw)),
        ]);

        let mut user = User { owned_car: None };
        car_dealer.rent_user(&mut user, "Audi".to_string());

        assert_eq!(car_dealer.cars[0].borrow().rent, true);
        // user.print_car();

        car_dealer.end_rental(&mut user);
        assert_eq!(car_dealer.cars[0].borrow().rent, false);
    }

    #[test]
    fn test_ex3() {
        let b_cow = Box::new(Cow);
        let b_dog = Box::new(Dog);

        let mut fc = Farmcell::new(b_cow);
        fc.insert(b_dog);

        assert_eq!(fc.make_sound(), "MooBau".to_string());
    }

    #[test]
    fn test_ex4() {
        let sl0 = PublicStreetLight::new(0, true, true);
        let sl1 = PublicStreetLight::new(1, true, false);
        let sl2 = PublicStreetLight::new(2, false, true);
        let sl3 = PublicStreetLight::new(3, false, false);

        let mut pi = PublicIllumination::new(vec![sl0, sl1, sl2, sl3]);

        // for lampione in pi.iter_mut() {
        //     println!("{:?}", lampione);
        // }

        
    }

    #[test]
    fn test_ex5() {
        let len = count_nodes::<
            Node<
            Node<
            Node<
            NullNode,
            NullNode,
            >,NullNode
            >,
            Node<
            Node<
            Node<
            Node<
            NullNode,
            NullNode
            >,
            NullNode
            >,
            Node<
            NullNode,
            NullNode
            >
            >,
            NullNode
            >
            >
            >();
        assert_eq!(len, 8);
    }

    #[test]
    fn test_ex6() {
        let mut b1 = EntangledBit::default();
        let mut b2 = EntangledBit::default();

        assert_eq!(b2.get(), false);
        b1.entangle_with(&mut b2);
        b1.set();
        assert_eq!(b2.get(), true);
    }
}
