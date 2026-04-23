#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person { name: "Alice".into(), age: 30 };
    let Person { name, ref age } = person; // moves `name`, borrows `age`
    println!("{}", name);
    println!("{}", age);
    println!("{:?}", person); // ERROR: `person` partially moved
}
