fn main() {
    let mut v = vec![0; 3];
    for (i, elem) in v.iter_mut().enumerate() {
        *elem = i * 2;
    }
    println!("{:?}", v);
}
