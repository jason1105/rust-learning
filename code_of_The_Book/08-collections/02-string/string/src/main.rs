// https://doc.rust-lang.org/book/ch08-02-strings.html

#[allow(unused_variables, unused_mut, unused_assignments)]
fn main() {
    // Listing 8-11
    let mut s = String::new();

    // Listing 8-12
    // use to_string() to create String object from string literal
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    // Listing 8-13
    // is equvlant to the code above
    let s = String::from("initial contents");

    // Listing 8-14
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Update string
    // Listing 8-15
    let mut s = String::from("foo");
    s.push_str("bar");

    // Listing 8-16: Using a string slice after appending its contents to a String
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);  // push_str() aren't borrowing s2
    println!("s2 is {}", s2);

    // Listing 8-17: Adding one character to a String value using push
    let mut s = String::from("lo");
    s.push('l');

    // Listing 8-18: Using the + operator to combine two String values into a new String value
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // + is a method which definition seems look like this : fn add(self, s: &str) -> String {


    // concatenate string using +
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3; // so unwieldy

    // concatenate string using format!()
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    let hello = "Здравствуйте";
    println!("{}", &hello[0..4]); // &hello[0..3] will cause error.

    let s =  "你好";
    let s1 = s;
    for (i,c) in s.chars().enumerate() {
        println!("index: {}, value: {}", i, c);
    }
    for (i,c) in s.bytes().enumerate() {
        println!("index: {}, value: {}", i, c);
    }
}

use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

    let mut map:HashMap<i32, i32> = HashMap::new();

    let len = nums.len();

    let mut i = 0;

    let mut ans: Vec<i32> = Vec::new();

    while i < len {

        let n = nums[i];

        if map.contains_key(&n) {
            ans.push(i as i32);
            ans.push(map[&n]);
            return ans;
        } else {
            map.insert(target-n as i32, i as i32);
        }

        i += 1;
    }

    return Vec::new();
}