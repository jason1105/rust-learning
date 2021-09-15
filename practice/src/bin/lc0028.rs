struct Solution;

/**
 * 还可以用CMP
 **/
fn main() {
    let mut haystack = String::from("mississippi");
    let mut needle = String::from("issip");
    let ans = Solution::str_str(haystack, needle);
    println!("ans = {:?}", ans);
}
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() == 0 {
            return 0;
        }
        let h = Vec::from(haystack);
        let n = Vec::from(needle);
        let mut i = 0;
        let mut j = 0;
        let mut candidate = 0;
        let mut ans: i32 = -1;

        while i < h.len() {
            if h[i] == n[j] {
                if j == 0 {
                    candidate = i;
                }
                if j == n.len() - 1 {
                    ans = candidate as i32;
                    break;
                }
                i += 1;
                j += 1;
            } else {
                if j == 0 {
                    i += 1;
                } else {
                    i = candidate + 1;
                    j = 0;
                }
                candidate = 0;
            }
        }

        ans
    }
}
