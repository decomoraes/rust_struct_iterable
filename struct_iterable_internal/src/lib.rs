pub trait Iterable {
    fn iter(&self) -> std::vec::IntoIter<(&'static str, &'_ dyn std::any::Any)>;
}