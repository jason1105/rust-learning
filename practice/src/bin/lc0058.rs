struct Solution;

fn main() {
    let mut s = String::from("a");
    let ans = Solution::length_of_last_word(s);
    println!("ans = {:?}", ans);
}
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let vec = Vec::from(s);
        let mut in_word: bool = false;
        let mut ans = 0;
        for i in 0..vec.len() {
            let i = vec.len() - i - 1;
            if in_word {
                if vec[i] == b' ' {
                    return ans;
                } else {
                    ans += 1;
                }
            } else {
                if vec[i] != b' ' {
                    in_word = true;
                    ans += 1;
                }
            }
        }

        ans
    }

    pub fn length_of_last_word2(s: String) -> i32 {
        s.bytes()
            .rev()
            .skip_while(|&c| c == b' ')
            .take_while(|&c| c != b' ')
            .count() as i32
    }

    pub fn length_of_last_word_chars(s: String) -> i32 {
        s.chars()
            .rev()
            .skip_while(|&b| b == ' ')
            .take_while(|&b| b != ' ')
            .count() as i32
    }

    pub fn length_of_last_word_loop(s: String) -> i32 {
        let mut s = s;
        let mut res = 0;

        while let Some(c) = s.pop() {
            if c == ' ' && res == 0 {
                continue;
            } else if c == ' ' && res != 0 {
                break;
            } else {
                res += 1;
            }
        }

        res
    }

    pub fn length_of_last_word_split(s: String) -> i32 {
        let ve: Vec<&str> = s.trim().rsplit(' ').collect();
        if ve.is_empty() {
            0
        } else {
            ve[0].len() as i32
        }
    }
}

// test solutions::p0058::bench::using_bytes ... bench:          39 ns/iter (+/- 10)
// test solutions::p0058::bench::using_chars ... bench:          44 ns/iter (+/- 1)
// test solutions::p0058::bench::using_loop  ... bench:          50 ns/iter (+/- 2)
// test solutions::p0058::bench::using_split ... bench:         408 ns/iter (+/- 13)
