struct Player {
    name: String,
}

impl Player {
    fn print(self) {
        println!("{}", self.name);
    }
}

fn main() {
    let p = Player {
        name: "Alice".into(),
    };
    p.print();
    p.print();
}
