fn main() {
    let nums = vec![1, 2, 3];
    let doubles: Vec<_> = nums.into_iter().map(|n| n * 2).collect();
    println!("{:?}", doubles);
}
