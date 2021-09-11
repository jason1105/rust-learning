struct Solution;

fn main() {
    let ans = Solution::is_valid(String::from("()"));
    println!("{}", ans);
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let chars = s.chars();
        let mut stack = vec![];

        for val in chars {
            match stack.is_empty() {
                true => stack.push(val),
                _ => {
                    let pair: (&char, char) = (stack.last().unwrap(), val);
                    match pair {
                        ('{', '}') => {
                            stack.pop();
                        }
                        ('[', ']') => {
                            stack.pop();
                        }
                        ('(', ')') => {
                            stack.pop();
                        }
                        _ => {
                            stack.push(val);
                        }
                    }
                }
            }
        }

        stack.is_empty()
    }

    pub fn another_is_valid(s: String) -> bool {
        let mut stack = vec![];
        for c in s.chars() {
            match c {
                '{' => stack.push('}'),
                '[' => stack.push(']'),
                '(' => stack.push(')'),
                '}' | ']' | ')' if Some(c) != stack.pop() => return false,
                _ => (),
            }
        }
        stack.is_empty()
    }

    // 作者：WalterWei
    // 链接：https://leetcode-cn.com/problems/valid-parentheses/solution/luo-ji-qian-xi-dai-ma-ji-zhu-shi-0-ms19-0rj2i/
    // 来源：力扣（LeetCode）
    // 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
}
