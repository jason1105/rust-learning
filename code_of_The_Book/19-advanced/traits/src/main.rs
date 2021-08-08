use std::cell::RefCell;
struct Counter{
    count: RefCell<i32>,
}

trait Ite<T> {
    fn next(& self) -> T;
}

impl Ite<i32> for Counter {
    fn next(& self) -> i32 {
        match self.count.borrow_mut() {
            mut x if *x < 5 => {
                *x += 1;
                *x
            },
            _ => 999
        }
    }
}

impl Ite<String> for Counter {
    fn next(& self) -> String {
        match self.count.borrow_mut() {
            mut x if *x < 5 => {
                *x += 1;
                x.to_string()
            },
            _ => "".to_string()
        }
    }
}

trait Add {
    type Output1;
}

struct MyBag;
struct Meter(i32);
struct Coordinate(i32, i32);
struct Person{name: String, age: i32,}

fn main() {
    let count = Counter{count: RefCell::new(0)};
    println!("{:?}", Ite::<i32>::next(&count));
    println!("{:?}", Ite::<String>::next(&count));
    
}
