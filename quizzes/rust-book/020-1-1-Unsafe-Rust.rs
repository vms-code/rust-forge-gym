fn main() {
    let mut v = Vec::with_capacity(4);
    for i in 0..3 {
        v.push(i);
    }
    let n = &v[0] as *const i32;
    v.push(4);
    println!("{}", unsafe { *n });
}
