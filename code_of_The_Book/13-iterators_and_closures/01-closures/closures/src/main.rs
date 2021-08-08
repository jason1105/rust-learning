// https://doc.rust-lang.org/book/ch13-01-closures.html

use std::thread;
use std::time::Duration;

// Listing 13-1: A function to stand in for a hypothetical calculation that takes about 2 seconds to run
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// Listing 13-3: The business logic that prints the workout plans based on the inputs and calls to the simulated_expensive_calculation function
fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

// Listing 13-4: Extracting the calls to simulated_expensive_calculation to one place and storing the result in the expensive_result variable
fn generate_workout2(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",expensive_result);
        println!(
            "Next, do {} situps!",expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",expensive_result);
        }
    }
}

// Listing 13-6: Calling the expensive_closure we’ve defined
fn generate_workout4(intensity: u32, random_number: u32) {
    let expensive_result = |intensity| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result(intensity));
        println!("Next, do {} situps!", expensive_result(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result(intensity));
        }
    }
}

// Listing 13-9: Defining a Cacher struct that holds a closure in calculation and an optional result in value
struct Cacher<T>
where T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
    map: HashMap<u32, u32>,
}

use std::collections::HashMap;

// Listing 13-10: The caching logic of Cacher
impl<T> Cacher<T>
where T: Fn(u32) -> u32,
{
    fn new (func: T) -> Cacher<T> {
        Cacher {
            calculation: func,
            value: None,
            map:  HashMap::new(),
        }
    }

    fn value(&mut self, intensity: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(intensity);
                self.value = Some(v);
                v
            }
        }
    }

    fn value2(&mut self, intensity: u32) -> u32 {

        match self.map.get(&intensity) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(intensity);
                self.map.insert(intensity, v);
                v
            }
        }
    }
}

fn generate_workout5(intensity: u32, random_number: u32) {

    // Listing 13-7: Adding optional type annotations of the parameter and return value types in the closure
    // let expensive_closure = |num: u32| -> u32 {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    // Listing 13-11: Using Cacher in the generate_workout function to abstract away the caching logic
    let mut expensive_result = Cacher::new(|intensity| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout5(simulated_user_specified_value, simulated_random_number);
}

#[test]
fn test() {
    let mut c = Cacher::new(|intensity| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    });

    assert_eq!(c.value2(1), 1);
    assert_eq!(c.value2(2), 2);
}