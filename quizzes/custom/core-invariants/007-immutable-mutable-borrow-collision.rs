fn main() {
    let mut a = 5;
    let r1 = &a;
    let r2 = &mut a;
    println!("{}, {}", r1, r2);
}
