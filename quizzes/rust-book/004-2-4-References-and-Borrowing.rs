fn get_first(v: &Vec<String>) -> &String {
    &v[0]
}

fn main() {
    let mut strs = vec![String::from("A"), String::from("B")];
    let first = get_first(&strs);
    if first.len() > 0 {
        strs.push(String::from("C"));
    }
}
