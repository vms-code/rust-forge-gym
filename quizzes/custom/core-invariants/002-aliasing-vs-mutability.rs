fn main() {
    let mut x = 5;
    let r1 = &x;
    let r2 = &mut x;
    println!("{}, {}", r1, r2);
}
