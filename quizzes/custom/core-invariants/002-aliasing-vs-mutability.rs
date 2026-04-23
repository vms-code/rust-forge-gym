fn main() {
    let mut x = 5;
    let r1 = &x;           // immutable borrow
    let r2 = &mut x;       // ERROR: cannot borrow `x` as mutable
    println!("{}, {}", r1, r2);
}