// https://doc.rust-lang.org/book/ch10-02-traits.html

use std::fmt::Display;

// Listing 10-12: A Summary trait that consists of the behavior provided by a summarize method
// pub trait Summary {
//     fn summarize(&self) -> String;  // 类似接口中的方法.
// }

// Listing 10-14: Definition of a Summary trait with a default implementation of the summarize method
pub trait Summary {
    fn summarize(&self) -> String {  // 可以为 trait 中的方法指定默认的实现.
        String::from("(Read more...)")
    }
}

// Listing 10-13: Implementing the Summary trait on the NewsArticle and Tweet types
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // we don't necessary implement method summarize if we want to use default implementation of summarize
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// we can use trait bound do the same thing as previous method of notify()
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Listing 10-16: Conditionally implement methods on a generic type depending on trait bounds
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> { // 通过 trait bound 限制 T 的范围.
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// 这是系统实现 ToString trait 的代码.
/*
impl<T: Display> ToString for T { // 为所有实现了 Display trait 的类型 T 实现 ToString trait
    // --snip--
}
*/

fn main() {
    println!("Hello, world!");
    
    let article = NewsArticle{
        headline: String::from("COVID has been in control"),
        location: String::from("Beijing"),
        author: String::from("Lee"),
        content: String::from("COVID ....."),
    };

    println!("Article {}", article.summarize());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    notify(&tweet);
}
