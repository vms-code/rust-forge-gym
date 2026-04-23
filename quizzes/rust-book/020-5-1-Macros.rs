macro_rules! manylet {
    ( $( $i:ident ),* = $e:expr ) => {
        $(
            let mut $i = $e;
        )*
    }
}
fn main() {
    let mut s = String::from("A");
    manylet!(x, y = s);
    x.push_str("B");
    println!("{x}{y}");
}
