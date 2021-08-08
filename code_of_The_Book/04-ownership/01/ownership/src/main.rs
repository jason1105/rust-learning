fn main1() {
    let mut s = String::from("hello");

    s.push_str(", world.");
    s = s + " I'm Lee.";

    println!("{}", s);

    // k.push_str(""); // 字面量字符串不能变
    // k = k + "1"; // 字面量字符串不能变

    println!("----------------------------");
    let mut s1 = String::from("ss");
    println!("s1: {}", s1); // s1: ss
    let mut s2 = s1;

    // println!("s1: {}", s1); // error.because reference to "ss" has moved to s1
    println!("s2: {}", s2); // s2: ss

    println!("----------------------------");
    let mut s1 = String::from("hello");
    println!("s1: {}", s1); // s1: hello
    s1.push_str(", world.");
    let mut s2 = s1.clone(); // use clone if you want to continuely use s1.
    println!("s1: {}", s1); // hello, world.
    println!("s2: {}", s2); // hello, world.

    s1.push_str(", I'm s1.");
    s2.push_str(", I'm s2.");
    println!("s1: {}", s1); // hello, world., I'm s1.
    println!("s2: {}", s2); // hello, world., I'm s2.

    println!("s1 compare to s2: {}", s1 == s2); // s1 compare to s2: false
}
fn main() {
    // reference save on stack
    // compound string save on heap
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here
                        //println!("{}", s); // error.

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward

    let s4 = String::from("s4");
    let t = foo(s4);
    println!("{}", t.0);
    // println!("s4 after call foo(): {}", s4); // error
    let s4 = t.0; // reassiment
    println!("s4 after call foo(): {}", s4);
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
fn foo(mut s: String) -> (String, usize) {
    let size = s.len(); // 模拟一个业务
    s.push_str(", foo.");
    (s, size) // 返回业务结果, 同时返回 s
}
