// https://doc.rust-lang.org/book/ch11-01-writing-tests.html

// Listing 11-5: Using the Rectangle struct and its can_hold method from Chapter 5
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Listing 11-8: Testing that a condition will cause a panic!
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    // Listing 11-9: Testing that a condition will cause a panic! with a particular panic message
    pub fn new2(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}


// mine
mod shape {
    #[derive(Debug)]
    pub struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        pub fn new(w : u32, h : u32) -> Self {
            Self {
                width: w,
                height: h
            }
        }
    }
}

// Listing 11-7: Testing the function add_two using the assert_eq! macro
pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    // format!("Hello {}!", name)
    format!("Hello!") // simulate a bug
}

// Listing 11-1: The test module and function generated automatically by cargo new
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // Listing 11-3: Adding a second test that will fail because we call the panic! macro
    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    // Listing 11-6: A test for can_hold that checks whether a larger rectangle can indeed hold a smaller rectangle
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    // mine
    #[test]
    fn larger_can_hold_smaller2() {
        let larger = shape::Rectangle::new (8, 7);

        let smaller = shape::Rectangle::new (5, 1);

        assert!(larger.can_hold(&smaller));
    }

    // Listing 11-7: Testing the function add_two using the assert_eq! macro
    #[test]
    fn test_add_two() {
        assert_eq!(add_two(0), 2);
        assert_eq!(add_two(1), 3);
        assert_eq!(add_two(2), 4);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"),
                "Greeting did not contain name, value was `{}`",
                result);
    }
    
    #[test]
    #[should_panic]
    fn guess_panic() {
        let result = Guess::new(101);
    }
    // Listing 11-9: Testing that a condition will cause a panic! with a particular panic message
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new2(200);
    }
}

// new test suit
#[cfg(test)]
mod tests2 {
    
    use super::*;

    #[test]
    #[should_panic]
    fn guess_panic() {
        let result = Guess::new(101);
    }
}