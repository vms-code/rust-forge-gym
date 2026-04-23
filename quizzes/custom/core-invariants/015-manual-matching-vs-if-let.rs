fn main() {
    let opt: Option<i32> = Some(42);
    if let Some(x) = opt {
        println!("{}", x);
    }
}
