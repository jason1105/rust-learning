fn main() {
    println!("Hello, world!");

    let a = "  hello  ";
    let b = a.trim_matches(' ');

    println!("a: {}", a);
    println!("b: {}", b);

    let num = 3;
    let sum = (1..num + 1).reduce(|a, b| a + b);
    println!("Sum = {}", sum.unwrap());

    let mut i1 = 1;
    let i2 = &mut i1;
    *i2 = 2;
    // i1 = 3;
    println!("i1 = {}", i1);

    let a = 1;
    let b = &a;
    let c = &b;
    println!("a = {}, b = {}, c = {}", a, b, c);

    #[derive(Debug, Copy, Clone)]
    struct S {
        a: i32,
        b: i32,
    };
    let s = S { a: 1, b: 2 };
    let s1 = s.a;

    println!("s = {:?}, s1 = {:?}", s, s1);
}

pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
    let freq = order
        .as_bytes()
        .iter()
        .enumerate()
        .fold(vec![0; 26], |mut freq, (i, b)| {
            freq[(b - b'a') as usize] = i;
            freq
        });
    words
        .iter()
        .map(|s| {
            s.as_bytes()
                .iter()
                .map(|&b| freq[(b - b'a') as usize])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .windows(2)
        .all(|words| words[0] <= words[1])
}
