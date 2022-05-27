use std::{fmt, mem};

use snippets::list::first;

/// https://rust-unofficial.github.io/too-many-lists/first-pop.html
///

fn main() {
    let x: Option<String> = Some("hey".to_owned());
    assert_eq!(x.as_deref(), Some("hey"));
} // t1
