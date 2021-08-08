// https://doc.rust-lang.org/book/ch11-02-running-tests.html

// Listing 11-10: Tests for a function that calls println!
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

// Listing 11-11: Three tests with three different names
pub fn add_two(a: i32) -> i32 {
    a + 2
}

// Listing 11-12: Testing a private function
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}


mod a;

fn say() {
    a::b::c::world();
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_world() {
        a::b::c::world();
    }

    #[test]
    fn test_hello() {
        a::hello();
    }

    // Listing 11-10: Tests for a function that calls println!
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    // #[test]
    // fn this_test_will_fail() {
    //     let value = prints_and_returns_10(8);
    //     assert_eq!(5, value);
    // }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    // Listing 11-12: Testing a private function
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}