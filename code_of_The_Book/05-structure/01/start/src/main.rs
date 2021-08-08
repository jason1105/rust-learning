// https://doc.rust-lang.org/book/ch05-01-defining-structs.html
// Listing 5-1
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // Listing 5-2
    // Listing 5-3
    /*
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    */

    let mut user1 = user_builder(String::from("Lee"), String::from("lee@mail.com"));

    println!("user1 name: {}", user1.username);

    user1.username = String::from("Bob");

    println!("user1 name: {}", user1.username);

    // Listing 5-7
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("user2 name: {}", user2.username);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let red = Color(255, 0, 0);
    let origin = Point(0, 0, 0);
}

fn user_builder(username: String, email: String) -> User {
    // Listing 5-4
    // User {
    //     username: String::from(name),
    //     email: String::from(email),
    //     sign_in_count: 5,
    //     active: true,
    // }

    // Listing 5-5
    User {
        username,
        email,
        sign_in_count: 5,
        active: true,
    }
}
