

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::{cmp::Ordering, io};
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);

        let mut map:HashMap<String,String> = HashMap::new();
        map.insert("hello".to_string(), "lee".to_string());
        println!("{}", map["hello"]);
    }
}

// https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
// Listing 7-1
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {

        }

        pub fn serve_order() {}

        fn take_payment() {}
    }
}

// Listing 7-8
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // Listing: 7-10
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

mod country {
    pub mod region {
        pub fn hello() {}
    }
}

pub use country::region;
pub use country::region as area;

pub use crate::front_of_house::hosting; // use module absolutely  // Listing 7-11 Listing 7-17
use back_of_house::Breakfast;       // use struct relatively 
use self::back_of_house::Appetizer;  // use enum relatively // Listing 7-12
use front_of_house::hosting::add_to_waitlist; // 可以使用 use 导入一个 function 到当前的module, 但很少这样用, 因为在使用该方法的地方可能看起来像是一个本地调用, 而实际上使用的却是其他 mod 中的内容

// Listing 7-3
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    add_to_waitlist(); // 这个方法定义在哪? 看上去像是定义在了本 mod 中, 但实际上是在其他 mod 中. so 避免这样用.

    // Listing 7-9
    // Order a breakfast in the summer with Rye toast
    let mut meal = Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    // Listing: 7-10
    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;
}

// Listing 7-15
use std::fmt;
use std::io;

// Listing 7-16
use std::io::Result as IoResult;

fn function1() -> fmt::Result {
    Ok(())
}

fn function2() -> io::Result<()> {
    Ok(())
}

// Listing 7-16
fn function3() -> IoResult<()> {
    Ok(())
}