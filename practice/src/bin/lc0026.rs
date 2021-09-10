fn main() {
    let mut nums = vec![];
    let ans = remove_duplicates(&mut nums);
    println!("{} {:?}", ans, nums);
}
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.sort();
    match nums.is_empty() {
        true => return 0,
        _ => {
            let mut i = 0;
            for j in 1..nums.len() {
                if nums[i] != nums[j] {
                    nums[i + 1] = nums[j];
                    i += 1;
                }
            }
            (i + 1) as i32
        }
    }
}
