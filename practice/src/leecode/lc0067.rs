pub struct Solution;

#[test]
// cargo test --lib lc0067 -- --show-output
fn lc0067() {
    assert_eq!(
        "101".to_string(),
        Solution::add_binary("10".to_string(), "11".to_string())
    );
    assert_eq!(
        "0".to_string(),
        Solution::add_binary("0".to_string(), "0".to_string())
    );
    assert_eq!(
        "1".to_string(),
        Solution::add_binary("0".to_string(), "1".to_string())
    );
    assert_eq!(
        "10".to_string(),
        Solution::add_binary("1".to_string(), "1".to_string())
    );
    assert_eq!(
        "100".to_string(),
        Solution::add_binary("11".to_string(), "1".to_string())
    );
}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let va = Vec::from(a);
        let vb = Vec::from(b);
        let n = match va.len() > vb.len() {
            true => va.len(),
            false => vb.len(),
        };

        let mut ans = String::new();

        let mut carry = 0;

        for i in 0..n {
            carry += if i < va.len() {
                va[va.len() - i - 1] - b'0'
            } else {
                0
            };
            carry += if i < vb.len() {
                vb[vb.len() - i - 1] - b'0'
            } else {
                0
            };
            println!("ans = {}, carry = {}", &ans, carry);
            ans.push((carry % 2 + b'0') as char);
            carry /= 2;
        }

        if carry == 1 {
            ans.push((carry + b'0') as char);
        }

        ans = ans.chars().rev().collect::<String>();

        println!("ans = {}", &ans);

        ans
    }
}
