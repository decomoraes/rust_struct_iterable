// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

pub trait Iterable {
    fn iter(&self) -> std::vec::IntoIter<(&'static str, &'_ dyn std::any::Any)>;
}

// pub trait Iterable {
//     fn iter(&self) -> dyn Iterator<Item = (&'static str, Box<dyn std::any::Any>)>;
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
