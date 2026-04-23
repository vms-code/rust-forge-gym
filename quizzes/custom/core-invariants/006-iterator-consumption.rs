fn main() {
    let v = vec![1, 2, 3];
    for x in v {
        print!("{}", x);
    }
    // v has been consumed (moved into the iterator)
    // println!("{:?}", v); // would cause "use of moved value"
}