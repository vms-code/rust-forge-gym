/// Adds a Display-able object into a vector of
/// Display trait objects
use std::fmt::Display;
fn add_displayable<T: Display>(v: &mut Vec<Box<dyn Display>>, t: T) {
    v.push(Box::new(t));
}
