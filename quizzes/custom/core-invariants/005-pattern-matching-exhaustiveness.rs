enum E {
    A,
    B,
}

fn main() {
    match E::A {
        E::A => println!("A"),
    }
}
