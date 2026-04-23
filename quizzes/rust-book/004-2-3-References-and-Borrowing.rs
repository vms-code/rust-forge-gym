fn main() {
    let mut s = String::from("Hello");
    let t = &mut s;
    /* here */
    t.push_str(" world");
    println!("{}", s);
}
