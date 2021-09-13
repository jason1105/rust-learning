struct Solution;

fn main() {
    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let val = 2;
    // let mut nums = vec![1];
    // let val = 1;
    let ans = Solution::remove_element(&mut nums, val);
    println!("ans = {:?}, vec = {:?}", ans, nums);
}

impl Solution {
    // 使用函数
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&x| x != val); // 函数入参是一个 i32 引用, 通过 &x 模式匹配它, 得到的 x 是 i32 类型
        nums.len() as i32
    }
    // 正向遍历
    pub fn remove_element3(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[j] != val {
                nums[i] = nums[j];
                i += 1;
            }
        }

        i as i32
    }
    // 逆向遍历
    pub fn remove_element2(nums: &mut Vec<i32>, val: i32) -> i32 {
        let len = nums.len();
        let mut i = 0;
        let mut j = len - 1;
        match len {
            0 => return 0,
            _ => {
                while i <= j {
                    if nums[j] != val {
                        nums.swap(i, j);
                        i += 1;
                    } else {
                        if j == 0 {
                            break;
                        }
                        j -= 1;
                    }
                }

                return i as i32;
            }
        }
    }
}
