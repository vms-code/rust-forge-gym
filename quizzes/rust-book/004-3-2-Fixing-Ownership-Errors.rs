fn copy_to_prev(v: &mut Vec<i32>, i: usize) {
    let n = &mut v[i];
    *n = v[i - 1];
}
fn main() {
    let mut v = vec![1, 2, 3];
    copy_to_prev(&mut v, 1);
}
