fn main() {
    let mut a = 5;
    let r1 = &a;       // immutable borrow
    let r2 = &mut a;   // ERROR: cannot borrow `a` as mutable because it is also borrowed as immutable
    println!("{}, {}", r1, r2);
}
