fn main() {
    let evens: Vec<_> = (0..6).filter(|x| x % 2 == 0).collect();
    println!("{:?}", evens);
}
