struct S(i32);

impl Drop for S {
    fn drop(&mut self) {
        print!("{}", self.0);
    }
}

fn main() {
    let a = S(1);
    let b = a;
    let _ = b;
}