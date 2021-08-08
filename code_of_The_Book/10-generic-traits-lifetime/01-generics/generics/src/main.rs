// https://doc.rust-lang.org/book/ch10-01-syntax.html

// Listing 10-4: Two functions that differ only in their names and the types in their signatures
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Listing 10-4: Two functions that differ only in their names and the types in their signatures
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Listing 10-5: A definition of the largest function that uses generic type parameters but doesn’t compile yet
fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_2<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_3<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Listing 10-7: The fields x and y must be the same type because both have the same generic data type T.
// struct Point<T> {
//     x: T,
//     y: T,
// }

// Listing 10-8: A Point<T, U> generic over two types so that x and y can be values of different types
struct Point_10_8<T, U> {
    x: T,
    y: U,
}

// Listing 10-11: A method that uses different generic types from its struct’s definition
impl<T, U> Point_10_8<T, U> {

    // Notice!! V and W are generics of method that are different with T and U defined in struct.
    fn mixup<V, W>(self, other: Point_10_8<V, W>) -> Point_10_8<T, W> {
        Point_10_8 {
            x: self.x,
            y: other.y,
        }
    }
}

// Listing 10-9: Implementing a method named x on the Point<T> struct that will return a reference to the x field of type T
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Listing 10-10: An impl block that only applies to a struct with a particular concrete type for the generic type parameter T
// 仅仅针对浮点数声明的方法.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Listing 10-4: Two functions that differ only in their names and the types in their signatures
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // let wont_work = Point { x: 5, y: 4.0 };
    
    // error because largest's generic bounds: PartialOrd + Copy
    // let num = vec!(integer, float);
    // let result = largest(&num);
    // println!("The largest Point is {}", result);

    // Listing 10-9
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    // Listing 10-11
    let p1 = Point_10_8 { x: 5, y: 10.4 };
    let p2 = Point_10_8 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);


}

