fn expect_none(x: Option<i32>) -> ! {
    match x {
        Some(n) => panic!("Expected none, found Some({n})"),
        None => (),
    }
}
fn main() {
    println!("{:?}", expect_none(None));
}
