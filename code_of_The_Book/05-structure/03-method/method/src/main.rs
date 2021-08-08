// https://doc.rust-lang.org/book/ch05-03-method-syntax.html
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Listing 5-13
// declare method in structure of Rectangle
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Listing 5-15
    fn can_hold(&self, another: &Rectangle) -> bool {
        (self.width > another.width && self.height >= another.height)
            || (self.width >= another.height && self.height >= another.width)
    }
}

// Listing 5-16:
impl Rectangle {
    fn square(length: u32) -> Rectangle {
        Rectangle {
            width: length,
            height: length,
        }
    }
}

fn main() {
    let rect = Rectangle {
        height: 2,
        width: 3,
    };
    let area = rect.area();
    println!("rect is: {:?}", rect);
    println!("rect is: {:#?}", rect);
    println!("area of rect is: {}", area);

    // 5-14
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let area3 = Rectangle::area(&rect3);
    println!("area3 {}", area3);

    let square = Rectangle::square(10);
    println!("square: {:?}", square);
}
