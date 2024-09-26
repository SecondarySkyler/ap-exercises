#![allow(unused_imports)]
use std::collections::HashMap;

use assignment2::{evaluate_expression, DoubleType, Expression, Operation};
use midterm1::{AirFleet, Airplane, AirplaneCompany, MaybePoint, Size, Student, University};

mod assignment1;
mod assignment2;
mod midterm1;
mod assignment3;

#[allow(dead_code)]
fn main() {

    // Assignment 1
    // let s = "Hello, world!";
    // println!("{}", assignment1::string_reverse(s));

    // let a = 10;
    // let b = 20;
    // println!("Between {} and {}, {} is bigger", a, b, assignment1::bigger(a, b));

    // let a = 10;
    // let b = 20.0;
    // let c = 30.0;
    // println!("{} * {} * {} = {}", a, b, c, assignment1::multiply(a, b, c));

    // let mass = 10.0;
    // println!("Energy of {} kg is {} J", mass, assignment1::energy(mass));

    // let v = vec![1, 2, 3, 4, 5];
    // let (max, min) = assignment1::max_min(&v);
    // println!("Max: {}, Min: {}", max, min);

    // let s = "test".to_string();
    // println!("{}", assignment1::lord_farquad(s));

    // let mut furniture: HashMap<String, f32> = HashMap::new();
    // furniture.insert("table".to_string(), 100.0);
    // furniture.insert("chair".to_string(), 50.0);
    // furniture.insert("bed".to_string(), 200.0);

    // println!("Price of table: {}", assignment1::get_price(&furniture, "table".to_string()));
    // println!("Price of sofa: {}", assignment1::get_price(&furniture, "sofa".to_string()));

    // let s: String = "some text".to_string();
    // let ss = assignment1::append(s.clone());
    // println!("Original: {}, Appended: {}", s, ss);

    // let v = vec![9, 10, 153, 154];
    // for i in v {
    //     if assignment1::is_armstrong(i) {
    //         println!("{} is an Armstrong number", i);
    //     } else {
    //         println!("{} is NOT an Armstrong number", i);
    //     }
    // }

    // let mut m: assignment1::Matrix = ((1, 2), (3, 4));
    // m = assignment1::transpose(m);
    // println!("{:?}", m);


    // Assignment 2
    // let mut v = vec![1, 2, 3, 4, 5];
    // assignment2::modify_odd(&mut v);
    // println!("{:?}", v);

    // let s = "Hello, world!".to_string();
    // let occurrences = assignment2::count_characters(s);
    // println!("{:?}", occurrences);

    // let v = vec![1, 2, 3, 4, 5];
    // if let Some((left, right)) = assignment2::split_at_value(&v, 6) {
    //     println!("{:?} {:?}", left, right);
    // } else {
    //     println!("Value not found");
    // }

    // let mut v1 = vec![1, 2, 3, 4, 5];
    // if let Some(max) = assignment2::max(&v1) {
    //     println!("The max in {:?} is: {}", &v1, max);
    // } else {
    //     println!("No max found");
    // }

    // assignment2::swap(&mut v1);
    // println!("swapped first and last element in {:?}", v1);

    // let is_sort = assignment2::is_sorted(&v1);
    // println!("Is {:?} sorted? {}", v1, is_sort);

    // let mut vs: Vec<String> = Vec::new();
    // assignment2::insert_if_longer(&mut vs, "Hello".to_string());
    // assignment2::insert_if_longer(&mut vs, "phraseoflengthten".to_string());
    // println!("The vector of strings is: {:?}", &vs);

    // let mut v = vec![15, 8, 9, 1, 78, 30, 69, 4, 10];
    // assignment2::pancake_sort(&mut v);
    // println!("{:?}", v);

    // let slice1 = &[1, 2, 3];
    // let slice2 = &[4, 5, 6];
    // let merged = assignment2::merge(slice1, slice2);
    // println!("Merged slices: {:?}", merged);

    // let _double_vec = vec![DoubleType::Int(1), DoubleType::Str("Hello".to_string())];
    
    // let op = Operation::Add;
    // let exp = Expression::Operation {
    //     left: Box::new(Expression::Number(1)),
    //     op: op,
    //     right: Box::new(Expression::Number(2))
    // };
    // match evaluate_expression(&exp) {
    //     Ok(result) => println!("Result: {}", result),
    //     Err(e) => println!("Error: {}", e)
    // }

    // let s = "hello";
    // println!("{}", midterm1::prev_str(s));
    // let s1 = "abcdefg_1234_ABCDEFG";
    // println!("{}", midterm1::prev_str(s1));

    // let v = vec!["How".to_string(), "are".to_string(), "you".to_string()];
    // let ordered = midterm1::order(v);
    // println!("{:?}", ordered);

    // let x = MaybePoint::new(Some(10), Some(20));
    // let y = MaybePoint{x:Some(10),y:None};

    // println!("{:?}", x.is_some());
    // println!("{:?}", y.is_some());
    // println!("{:?}", x.maybe_len());
    // println!("{:?}", y.maybe_len());

    // let s = Size::new(5.7, 1.2);

    // println!("{:?}", s.area());
    // println!("{:?}", s.compare(&Size::new(8.9, 10.)));
    // println!("{:?}", s.compare(&Size::new(1.8, 0.1)));
    // println!("{:?}", s.compare(&Size::new(5.7, 1.2)));

    // let mut fleet = AirFleet{
    //     fleet: Vec::new(),
    // };

    // let airplane1 = Airplane{
    //     company: AirplaneCompany::Airbus,
    //     model: "A380".to_string(),
    // };

    // let airplane2 = Airplane{
    //     company: AirplaneCompany::Boeing,
    //     model: "747".to_string(),
    // };

    // let airplane3 = Airplane{
    //     company: AirplaneCompany::Airbus,
    //     model: "A320".to_string(),
    // };

    // fleet.add_airplane(airplane1);
    // fleet.add_airplane(airplane2);
    // fleet.add_airplane(airplane3);

    // println!("{:?}", fleet.search_airplane("A380"));
    // println!("{:?}", fleet.search_airplane("747"));
    // println!("{:?}", fleet.search_airplane("A320"));
    // println!("{:?}", fleet.search_airplane("A330"));

    // let s1 = Student::new("marco", 1);
    // let s2 = Student::new("anto", 2);
    // let s3 = Student::new("anna", 3);
    // let mut university = University::new("Trento", &vec![s1, s2, s3]);

    // println!("{}", university);

    // println!("{}", university.remove_student(1).unwrap().id);

    
    
  
}


