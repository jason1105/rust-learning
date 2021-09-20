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
        let vec: Vec<char> = s.chars().filter(|c| c.is_ascii_alphanumeric()).collect();

        let len = vec.len();
        if len == 0 || len == 1 {
            return true;
        }

        for i in 0..len {
            let j = len - i - 1;
            if !vec[i].eq_ignore_ascii_case(&vec[j]) {
                return false;
            }
        }

        true
    }
    pub fn is_palindrome2(s: String) -> bool {
        // native rust，但是，效率差不多
        // let mut iter = s.chars().filter(|c| c.is_alphanumeric()).map(|c| c.to_ascii_lowercase());
        // iter.clone().eq(iter.rev())

        // 逐个识别字符串内容，保留数字和字符
        let mut chars = s.chars().filter(|c| c.is_alphanumeric());

        while let (Some(c1), Some(c2)) = (chars.next(), chars.next_back()) {
            // 转换成小写后对比
            // pub fn eq_ignore_ascii_case(&self, other: &char) -> bool {
            //     self.to_ascii_lowercase() == other.to_ascii_lowercase()
            // }
            if !c1.eq_ignore_ascii_case(&c2) {
                return false;
            }
            // 这里还可以优化, while 无需遍历整个字符串, 遍历一半即可
        }
        true
    }
}
