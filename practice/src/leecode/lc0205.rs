pub struct Solution;

#[test]
// cargo test --lib lc0058 -- --show-output
fn lc0205() {
    assert_eq!(
        true,
        Solution::is_isomorphic("egg".to_string(), "add".to_string())
    );
    assert_eq!(
        false,
        Solution::is_isomorphic("foo".to_string(), "bar".to_string())
    );
    assert_eq!(
        true,
        Solution::is_isomorphic("paper".to_string(), "title".to_string())
    );
    assert_eq!(
        false,
        Solution::is_isomorphic("badc".to_string(), "baba".to_string())
    );
}

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        use std::collections::HashMap;

        if s.len() == 0 {
            return true;
        }

        let mut map_a_2_b = HashMap::new();
        let mut map_b_2_a = HashMap::new();

        let iter = s.chars().zip(t.chars());

        let mut ans = true;

        iter.for_each(|(a, b)| {
            let value_b = map_a_2_b.entry(a).or_insert(b);
            if value_b != &b {
                ans = false;
            }
            let value_a = map_b_2_a.entry(b).or_insert(a);
            if value_a != &a {
                ans = false;
            }
        });

        ans
    }
}
