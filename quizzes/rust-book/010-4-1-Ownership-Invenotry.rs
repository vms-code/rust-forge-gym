/// Returns the n-th largest element in a slice
fn find_nth<T: Ord + Clone>(elems: &[T], n: usize) -> T {
    elems.sort();
    let t = &elems[n];
    return t.clone();
}
