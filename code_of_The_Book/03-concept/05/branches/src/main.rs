fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;

    // assignment by if
    let x = if condition { 5 } else { 3 };

    // assignment by loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // while
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    //

    let mut count = 10;

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOUT!");
}
