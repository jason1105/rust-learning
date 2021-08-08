use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101); // or (1..=100)

    // println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue, // _表示所有的错误
        };

        println!("You guessed: {}", guess);

        /*
         math pattern {
             arm => action,
             arm => action
        }
        */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),  // arm
            Ordering::Greater => println!("Too big!"), // arm
            Ordering::Equal => {
                println!("You win!");
                break;
            } // arm
        }
    }
}

// 把需要校验的值和其校验逻辑封装成为一个整体. struct + impl
// Listing 9-10: A Guess type that will only continue with values between 1 and 100
mod {
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
    
        pub fn value(&self) -> i32 {
            self.value
        }
    }
}