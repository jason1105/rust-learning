struct Solution;

fn main() {
    let mut s = String::from("a");
    let ans = Solution::longest_palindrome(s);
    println!("ans = {:?}", ans);
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut max = 0;
        let (start, end): (usize, usize) = (0, 0);
        let mut ans: (usize, usize) = (0, 0);
        let vec = Vec::from(s.clone());
        for i in 0..vec.len() {
            let (start, end) = Solution::find_max(i, i, true, &vec);
            //println!("return: {} - {}", start, end);
            if end - start + 1 > max {
                max = end - start + 1;
                ans.0 = start;
                ans.1 = end;
            }
        }
        // println!("ans: {} - {}", ans.0, ans.1);
        String::from(&s[ans.0..ans.1 + 1])
    }

    pub fn find_max(s: usize, e: usize, same: bool, string: &Vec<u8>) -> (usize, usize) {
        println!("find {} to {}", s, e);
        // if Solution::consist_same(&string[s..e + 1]) {
        if same {
            if s > 0 && string[s - 1] == string[s] {
                return Self::find_max(s - 1, e, true, string);
            }
            if e + 1 < string.len() && string[e + 1] == string[s] {
                return Self::find_max(s, e + 1, true, string);
            }
        }

        if s > 0 && e + 1 < string.len() && string[s - 1] == string[e + 1] {
            return Self::find_max(s - 1, e + 1, false, string);
        }

        (s, e)
    }

    pub fn consist_same(str: &str) -> bool {
        let first = str.chars().nth(0);
        for i in 0..str.len() {
            if str.chars().nth(i) != first {
                return false;
            }
        }
        true
    }
}
