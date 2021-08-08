use std::fmt::Display;
fn main() {

    // Listing 10-17: An attempt to use a reference whose value has gone out of scope
    {
        let r; // this is ok because rust will check it to ensure not null before using it.

        {
            let x = 5;
            // r = &x; // error, it's a reference
            r = x; // ok, it's a copy 
        } // here, reference become invalid as lifetime over.

        println!("r: {}", r);
    }

    // Listing 10-18: Annotations of the lifetimes of r and x, named 'a and 'b, respectively
    {
    //     let r;                // ---------+-- 'a
    //                           //          |
    //     {                     //          |
    //         let x = 5;        // -+-- 'b  |
    //         r = &x;           //  |       |
    //     }                     // -+       |
    //                           //          |
    //     println!("r: {}", r); //          |
    }                            // ---------+
 
    // Listing 10-19: A valid reference because the data has a longer lifetime than the reference
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
    
    // Listing 10-20: A main function that calls the longest function to find the longer of two string slices
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // Listing 10-23: Using the longest function with references to String values that have different concrete lifetimes
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // Listing 10-24: Attempting to use result after string2 has gone out of scope
    // following will compile error.
    /*
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str()); 
    } // string2.as_str() is invalid so result is invalid too
    println!("The longest string is {}", result); // result is invalid.
    */

    // Listing 10-25: A struct that holds a reference, so its definition needs a lifetime annotation
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
// Listing 10-21: An implementation of the longest function that returns the longer of two string slices but does not yet compile
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// Listing 10-22: The longest function definition specifying that all the references in the signature must have the same lifetime 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn biggest(x: &i32, y: &i32) -> &i32 {
//     if (x > y){
//         x
//     } else {
//         y
//     }
// }

fn longest3<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// Listing 10-25: A struct that holds a reference, so its definition needs a lifetime annotation
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}