fn is_equal<T: Eq>(t1: &T, t2: &T) -> bool {
    t1 == t2
}
fn main() {
    println!("{}", is_equal("Hello", "world"));
}
