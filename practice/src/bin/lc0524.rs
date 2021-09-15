struct Solution;

fn main() {
    let mut s = String::from("abpcplea");
    let mut dictionary = vec!["ale", "apple", "monkey", "plea"];
    let dictionary = dictionary
        .iter()
        .copied()
        .map(|x| String::from(x))
        .collect();
    let ans = Solution::find_longest_word(s, dictionary);
    println!("ans = {:?}", ans);
}
impl Solution {
    pub fn find_longest_word(s: String, mut dictionary: Vec<String>) -> String {
        dictionary.sort_by(|a, b| {
            if a.len() == b.len() {
                a.cmp(b)
            } else {
                b.len().cmp(&a.len())
            }
        });
        let mut ans: String = String::from("");

        let s = Vec::from(s);
        let mut j: usize; // index of string
        for string in dictionary {
            let string = Vec::from(string);
            j = 0; // reset
            for i in 0..s.len() {
                if s[i] == string[j] {
                    j += 1;
                }
                if j == string.len() {
                    ans = String::from_utf8(string).unwrap();
                    return ans;
                }
            }
        }
        ans
    }
}
