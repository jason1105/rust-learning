fn main() {
    let s1 = String::from("hello"); // use mut if you want to change

    let len = calculate_length(&s1); // use &mut if you want to change

    println!("The length of '{}' is {}.", s1, len);

    // List 4-6
    let mut s = String::from("hello");

    change(&mut s);
    change2(&mut s);

    println!("s is {}.", s);

    let s = String::from("hello, world.");
    //let s = "hello, world.";
    println!("{}", &s[0..3]);

    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[4]; // & means borrow, 借用
    println!("{}",third);
    // v.clear(); // borrow is equivalent to vec.clear(self)
    println!("{}",third);

}

// use mut if you want to change
fn calculate_length(s: &String) -> usize {
    s.len()
}

// List 4-6
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}
