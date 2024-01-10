use std::{fmt, mem, vec};

use snippets::list::first;

/// https://rust-unofficial.github.io/too-many-lists/first-pop.html
///

pub fn reorder_spaces(text: String) -> String {
    let arr = text
        .split_ascii_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    print!("{:?}", arr);
    print!("{:?}", arr.join("--"));
    let (word_cnt, space_cnt) = (
        arr.len(),
        text.as_bytes().iter().filter(|x| **x == ' ' as u8).count(),
    );
    let repeat = if word_cnt <= 1 {
        0
    } else {
        space_cnt / (word_cnt - 1)
    };
    format!(
        "{}{}",
        arr.join(" ".repeat(repeat).as_str()),
        " ".repeat(space_cnt - repeat * (word_cnt - 1))
    )
}

fn main() {
    let v1 = Some(1);
    let v2 = v1.map(|v| 2);
    assert_eq!(v2, Some(2));

    let v1 = Some(1);
    let v2 = v1.and_then(|v| Some(3));
    assert_eq!(v2, Some(3));

    let ans = reorder_spaces("  help".to_string());
    print!("\"{}\"", ans);

    let vec = vec![
        String::from("zyc"),
        String::from("zbz"),
        String::from("zya"),
    ];
    longest_common_prefix(vec);

    let a = [0i32, 1, 2];

    let mut iter = a.iter().filter(|x| x.is_positive());

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);

    println!("{:?}", a);

    let string = String::from("abc");
    let stringg: &str = &string;

    stringg.clone();

    let mut string2 = string.clone();

    println!(
        "string: {:p}, stringg: {:p}, string2: {:p}",
        string.as_ptr(),
        stringg,
        string2.as_ptr()
    );

    // unsafe {
    //     let string3 = String::from_raw_parts(0x55c1b3cb69d0 as *mut u8, 3, 3);
    //     println!("string3 = {}", string3);
    // }

    {
        use core::cell::Cell;

        trait SomeTrait {
            fn do_some(&self);
        }

        #[derive(Debug)]
        struct S1 {
            val: u32,
        }

        #[derive(Debug)]
        struct S2 {
            val: Cell<u32>,
        }

        impl SomeTrait for S1 {
            fn do_some(&self) {
                // I can't change anything.
            }
        }

        impl SomeTrait for S2 {
            fn do_some(&self) {
                self.val.set(999);
            }
        }

        let s1 = S1 { val: 111 };
        let s2 = S2 {
            val: Cell::new(222),
        };

        println!("{:?}, {:?}", s1, s2);

        s1.do_some();
        s2.do_some();

        println!("{:?}, {:?}", s1, s2);

        /*
           S1 { val: 111 }, S2 { val: Cell { value: 222 } }
           S1 { val: 111 }, S2 { val: Cell { value: 999 } }
        */
    }

    pub fn first_palindrome(words: Vec<String>) -> String {
        pub fn isPalindrome(s: &str) -> bool {
            let s2 = s.chars().rev().collect::<String>();
            s == s2
        }

        words
            .into_iter()
            .find(|x| *x == x.chars().rev().collect::<String>())
            .unwrap_or(String::from(""))
    }

    {
        /**
         * "2222222" 这是一个 dst 类型的变量 str, 它在内存中的样子就是这样, 一个一个并排在一起, 没有长度信息.
         *
         *
         */
        let mut v = String::from("122223");
        let rs = v.as_str();
        // print!("{}", rs[0]);
        let arr = [0; 2];
        print!("{:?}", &(&arr)[..]);
    }
} // t1

pub fn reverse_left_words(s: String, n: i32) -> String {
    // let s: &str = s.as_ref();
    let n = n as usize;
    format!("{}{}", &s[0..n], &s[n..])
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let d: String = strs
        .iter()
        .max()
        .unwrap()
        .chars()
        .zip(strs.iter().min().unwrap().chars())
        .take_while(|x| x.0 == x.1)
        .map(|x| x.0)
        .collect();
    println!("{:?}", d);
    return String::new();
}

struct Book {
    a: [i8],
}

pub fn trim_mean(mut arr: Vec<i32>) -> f64 {
    let dels = arr.len() / 20;
    arr.sort();
    (arr[dels..arr.len() - dels].iter().sum::<i32>()) as f64
        / (arr.len() - dels * 2) as f64
}

// pub fn longest_common_prefix2(strs: Vec<String>) -> String {
//     return strs
//         .iter()
//         .max()
//         .zip(strs.iter().min())
//         .take_while(|x| x.0 == x.1)
//         .collect();
// }

// 1147
pub fn longest_decomposition(text: String) -> i32 {
    let len = text.len();
    if len == 0 {
        return 0;
    }

    for i in 1..len / 2 {
        if text[0..i] == text[len - i - 1..len - i] {
            return 2 + longest_decomposition(text[i..len - i - 1].to_string());
        }
    }

    1
}

// 1624
pub fn max_length_between_equal_characters(s: String) -> i32 {
    s.chars()
        .enumerate()
        .map(|(i, x)| {
            let len = s
                .chars()
                .enumerate()
                .filter(|(_, y)| x == *y)
                .map(|(j, _)| j as i32 - i as i32 - 1)
                .max()
                .or(Some(-1));
            len.unwrap()
        })
        .max()
        .or(Some(-1))
        .unwrap() as i32
}

//1636
pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;

    let mut freq = HashMap::new();

    nums.iter().for_each(|x| {
        match freq.get(x) {
            Some(f) => freq.insert(x, f + 1),
            None => freq.insert(x, 0),
        };
    });

    let mut ans = nums.to_vec();

    ans.sort_by(|a, b| {
        if freq.get(a) == freq.get(b) {
            b.cmp(a)
        } else {
            freq.get(a).cmp(&freq.get(b))
        }
    });
    ans
}

//
pub fn frequency_sort2(mut nums: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut count = HashMap::new();

    for &num in &nums {
        *count.entry(num).or_insert(0) += 1;
    }

    nums.sort_unstable_by(|a, b| b.cmp(a));
    nums.sort_by_key(|k| count.get(k).unwrap());

    nums
}

pub fn ttse() {
    let strings = vec![String::from("abc"), String::from("123")];

    strings
        .iter()
        .map(|x| {
            //let d: () = x;  // &String
        })
        .count();

    let mut string = String::from("ab");
    // assert_eq!(Some('你'), string.chars().next());
    string.chars().map(|mut x| x = 'c').count();
    println!("{}", string);

    // let chars = vec![''];
    // chars.iter().map(|x| {
    //     let d : () = x;
    // });

    // let ns = vec![1, 2, 3];
    // ns.iter().map(|x| {
    //     let d: () = x;
    // });
}

#[test]
fn testasd() {
    ttse();
}
