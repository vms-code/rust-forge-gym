use std::rc::Rc;
fn main() {
    let n = Rc::new(1);
    let mut n2 = Rc::clone(&n);
    *n2 += 1;
    println!("{}", n);
}
