fn greet(name: &String) {
    println!("Hello, {}!", name);
}

fn main() {
    let name = String::from("Alice");
    greet(&name); // borrows — name is still usable
    greet(&name); // borrows again — still valid
    println!("name is still: {}", name);
}
