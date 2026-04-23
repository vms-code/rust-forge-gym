fn make_ref<'a>() -> &'a String {
    let s = String::from("hello");
    &s // ERROR: `s` does not live long enough — returns reference to local data
}

fn main() {
    let _r = make_ref();
}
