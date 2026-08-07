fn greet(name: &String) {
    println!("Hello, {}!", name);
}

fn main() {
    let name = String::from("Alice");
    greet(&name);
    greet(&name);
    println!("name is still: {}", name);
}
