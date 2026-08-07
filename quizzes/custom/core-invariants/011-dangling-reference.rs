fn make_ref<'a>() -> &'a String {
    let s = String::from("hello");
    &s
}

fn main() {
    let _r = make_ref();
}
