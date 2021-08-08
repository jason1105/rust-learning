// https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html

// Listing 12-1: Collecting the command line arguments into a vector and printing them
use std::env;
use std::process;
use jason_minigrep::Config; // crate resolves the path relative to the current crate

// CASE_INSENSITIVE=1 cargo run ar poem.txt
// cargo run ar poem.txt
fn main() {
    // let args: Vec<String> = env::args().collect(); // use instead args_os() to process non-unicode


    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });

    // Listing 13-27: Changing the body of Config::new to use iterator methods
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("{:?}", args);
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    // run(config).unwrap_or_else(|err| {
    //     println!("Problem running: {}", err);
    //     process::exit(1);
    // });

    if let Err(e) = jason_minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

