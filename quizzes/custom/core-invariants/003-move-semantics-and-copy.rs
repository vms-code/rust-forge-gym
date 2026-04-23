fn main() {
    let x: i32 = 1;
    let _y = x;          // x is Copy → y is a copy, x remains valid
    print!("{}", x);

    let a = String::from("2");
    let b = a;          // a is moved into b → a is no longer valid
    print!("{}", b);
}