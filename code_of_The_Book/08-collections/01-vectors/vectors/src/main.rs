// https://doc.rust-lang.org/book/ch08-01-vectors.html

fn main() {
    let v : Vec<i32> = Vec::new();
    let v = vec![1,2,3];
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    // Listing 8-5
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(5) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // Listing 8-7:
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0]; // immutable borrow

    // v.push(6); // mutable borrow , error

    println!("The first element is: {}", first);

    // Listing 8-8
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // Listing 8-9
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    let mut v = vec![100, 32, 57];
    v.push(200);
    for i in &mut v {
        if *i == 100 {
            // v.push(200); // error, senond mutable borrow
            *i += 50;
        }
    }
    println!("{:?}", v);

    // Listing 8-10
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        match i {
            SpreadsheetCell::Int(num) => println!("Number: {}", num),
            SpreadsheetCell::Text(t) => println!("Text: {}", t),
            SpreadsheetCell::Float(f) => println!("Float: {}", f),
            _ => (),
        }
    }

    // equals above
    for i in &row {
        if let SpreadsheetCell::Int(num) = i {
            println!("Number: {}", num);
        } else if let SpreadsheetCell::Text(text) = i {
            println!("Text: {}", text);
        } else if let  SpreadsheetCell::Float(f) = i {
            println!("Float: {}", f);
        } 
    }

    // println!("{}", row[0]);
}
