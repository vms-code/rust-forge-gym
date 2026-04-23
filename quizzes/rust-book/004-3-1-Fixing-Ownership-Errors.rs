fn main() {
    let s = String::from("Hello world");
    let s_ref = &s;
    let s2 = *s_ref;
    println!("{s2}");
}
