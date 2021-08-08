// https://doc.rust-lang.org/book/ch04-03-slices.html
fn main() {
    let s = String::from("hello world.");
    let first = first_word(&s);
    // s.clear(); // runtime error.
    println!("First word in s: {}", first);

    let s = "hello, world.";
    println!("{}", &s[0..1]);
}

fn first_word(string: &str) -> &str {
    let bytes = string.as_bytes();

    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &string[..i];
        }
    }

    &string[..]
}
