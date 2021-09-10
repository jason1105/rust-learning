fn main() {
    let mut nums = vec![1, 1, 1];
    let ans = remove_duplicates(&mut nums);
    println!("{}", ans);
}
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.sort();
    let len = nums.len();
    if len == 0 || len == 1 {
        return len as i32;
    }
    let (mut i, mut j) = (0, 1);
    let mut r = 0;
    //println!("{}", i);

    while i < len && j < len {
        if r == len - 1 {
            break;
        }
        if nums[i] != nums[j] {
            if nums[i] > nums[j] {
                break;
            }
            i += 1;
            j += 1;
            continue;
        }
        println!("Rotate {:?} i = {}, j = {}", nums, i, j);
        if j + 1 == len {
            break;
        }
        nums[j..len].rotate_left(1);
        r += 1;
        println!("{:?}", nums);
    }

    //println!("{:?}", nums);

    (i + 1) as i32
}
