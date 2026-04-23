fn dangle() -> &String {
    let s = String::from("hello");
    &s  // ERROR: `s` does not live long enough
}

fn main() {}