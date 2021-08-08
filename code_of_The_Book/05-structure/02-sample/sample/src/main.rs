// https://doc.rust-lang.org/book/ch05-02-example-structs.html
// Listing 5-12
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width = 2;
    let height = 3;

    // approach 1
    let area = area(width, height);
    println!("area is: {}", area);

    // approach 2
    let area = area2((width, height));
    println!("area2 is: {}", area);

    // approach 3
    let rect = Rectangle {
        height: height,
        width: width,
    };
    let area = area3(&rect); // use reference, so we can use variable rect in future.
    println!("rect is: {:?}", rect);
    println!("rect is: {:#?}", rect);
    println!("area3 is: {}", area);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// Listing 5-9
fn area2(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

// Listing 5-10
fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
