use std::fmt::Debug;
fn main() {
    let n = 1;
    let s = String::from("Hello");
    let v: Vec<&dyn Debug> = vec![&n, &s];
    let n_ref = v[0] as &i32;
    println!("{}", n_ref + 1);
}
