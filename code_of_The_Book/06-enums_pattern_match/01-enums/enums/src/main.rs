// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
// https://doc.rust-lang.org/book/ch06-02-match.html
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {}

struct IpAddr {
    kind: IpAddrKind,
    addr: String,
}

enum IpAddrEnum {
    V4(String),
    V6(String),
}

// Listing 6-2:
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },    // anonimous struct
    Write(String),              // string (tuple struct)
    ChangeColor(i32, i32, i32), // 3 int (tuple struct)
}

impl Message {
    fn call(&self) {
        println!("self {:?}", self);
    }
}

// -------------------------------------
// Listing 6-3
// Listing 6-4
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("You can do something here.");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter of state {:?}", state);
            25
        }
    }
}
// -------------------------------------

// Listing 6-5
fn plus_one(o: Option<i32>) -> Option<i32> {
    match o {
        Some(i) => Some(i + 1),
        None => None,
    }
}

// 必须匹配 i32 所有范围, 所以这个例子是错误的.
fn check_number(n: i32) {
    match n {
        1 => println!("Lucky number!"), // 1 is lucky number
        _ => (),                        // else do nothing
    }
}

fn main() {
    let homeAddr = IpAddr {
        kind: IpAddrKind::V4,
        addr: String::from("127.0.0.1"),
    };

    let officeAddr = IpAddrEnum::V4(String::from("127.0.0.1"));

    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 2 };
    let m3 = Message::Write(String::from("hello"));
    let m4 = Message::ChangeColor(255, 0, 0);

    println!("m1 {:?}", m1);
    println!("m2 {:?}", m2);
    println!("m3 {:?}", m3);
    println!("m4 {:?}", m4);

    m3.call();

    let a = Some("hello");
    let b = Some(5);
    let c: Option<i32> = None;
    let aa = a.expect("");
    println!("a: {}", aa);
    // let cc = c.expect("error, is none value."); // cause error because None.
    // println!("c: {}", cc);

    // Listing 6-3
    let c1 = Coin::Penny;
    let c1_value = value_in_cents(c1);
    println!("c1 value: {}", c1_value);

    let alabama = Coin::Quarter(UsState::Alabama);
    let alabama_value = value_in_cents(alabama);
    println!("alabama value: {}", alabama_value);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let five = plus_one(five);
    println!("add 1 to Some(5): {:?}", five);

    println!("i32::MIN: {}", i32::MIN);

    println!("i32::MAX: {}", i32::MAX);

    check_number(1);
    check_number(2);

    let mut count = 0;
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        count = count + 1;
    }
    println!("count: {}", count);
}
