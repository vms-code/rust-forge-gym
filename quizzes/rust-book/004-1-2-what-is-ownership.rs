fn add_suffix(mut s: String) -> String {
    s.push_str(" world");
    s
}
fn main() {
    let s = String::from("hello");
    let s2 = add_suffix(s);
    println!("{}", s2);
}
