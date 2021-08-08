fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; // compile error
    println!("The value of x is: {}", x);

    let x = 3;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    // int
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // char
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x in tup: {}", x);
    println!("x in tup: {}", tup.0);

    // array
    let a = [1, 2, 3, 4, 5];

    let a = [3; 5];
    let a: [i32; 5] = [3; 5];
}
