fn main() {
    let mut nums = vec![1, 1, 1];
    let ans = remove_duplicates(&mut nums);
    println!("{}", ans);
}
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}
