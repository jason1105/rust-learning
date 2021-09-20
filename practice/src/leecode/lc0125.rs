pub struct Solution;

#[test]
// cargo test --lib lc0067 -- --show-output
fn lc0125() {
    assert_eq!(
        true,
        Solution::is_palindrome("A man, a plan, a canal: Panama".to_string())
    );
    println!("1");
    assert_eq!(false, Solution::is_palindrome("race a car".to_string()));
    println!("2");
    assert_eq!(true, Solution::is_palindrome("a".to_string()));
    println!("3");
    assert_eq!(true, Solution::is_palindrome("".to_string()));
    println!("4");
    assert_eq!(true, Solution::is_palindrome("a.".to_string()));
    println!("5");
}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let len = s.len();
        if len == 0 || len == 1 {
            return true;
        }

        let vec: Vec<char> = s.to_lowercase().chars().collect();

        let (mut i, mut j) = (0, len - 1);

        while i < j {
            while i < len && !vec[i].is_ascii_alphanumeric() {
                i += 1;
            }
            while !vec[j].is_ascii_alphanumeric() {
                if j > 0 {
                    j -= 1;
                } else {
                    break;
                }
            }
            if i > j {
                return true;
            }

            if i <= j && vec[i] == vec[j] {
                i += 1;
                if j > 0 {
                    j -= 1;
                } else {
                    break;
                }
            } else {
                return false;
            }
        }

        true
    }
}
