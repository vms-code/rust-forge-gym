struct Player {
    name: String,
}

impl Player {
    fn print(self) {
        println!("{}", self.name);
    }
}

fn main() {
    let p = Player { name: "Alice".into() };
    p.print(); // p is moved here
    p.print(); // ERROR: use of moved value `p`
}
