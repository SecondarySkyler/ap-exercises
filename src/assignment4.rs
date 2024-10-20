#![allow(dead_code)]
use std::result;

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

#[cfg(test)]
mod test4 {
    use super::*;

    #[test]
    fn test_find_equal() {
        assert_eq!(find_equal("hello", "world"), None);
        assert_eq!(find_equal("hello", "hell"), Some(("he", "he")));
    }
}