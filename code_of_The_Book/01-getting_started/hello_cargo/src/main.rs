fn main() {
    println!("Hello, world!");

    let a = "  hello  ";
    let b = a.trim_matches(' ');

    println!("a: {}", a);
    println!("b: {}", b);

    let num = 3;
    let sum = (1..num+1).reduce(|a, b| a + b);
    println!("Sum = {}", sum.unwrap());
}
