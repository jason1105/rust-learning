mod org;

#[allow(unused_mut, unused_variables)]
fn main() {
    // Listing 8-20
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Listing 8-21
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    // Listing 8-22: Showing that keys and values are owned by the hash map once they’re inserted
    let mut field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, &field_value);
    let mut field_name = "Hello".to_string();
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    println!("{:?}", map); // error    value borrowed here after move

    let mut s = String::from("hello");
    let r = &s;
    println!("r: {}", r);
    let s = String::from("world");
    let s = "...".to_string();
    println!("r: {}", r);

    // Listing 8-23: Accessing the score for the Blue team stored in the hash map
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    if let Some(v) = scores.get(&team_name) { // use &str
        println!("key: {}, value: {}", &team_name, v);
    }

    if let Some(v) = scores.get("Blue") { // use literal, a literal is a type of &str
        println!("key: {}, value: {}", &team_name, v);
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    
    // Listing 8-24: Replacing a value stored with a particular key

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // Listing 8-25: Using the entry method to only insert if the key does not already have a value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);


    // Listing 8-26: Counting occurrences of words using a hash map that stores words and counts

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let mut nums = vec![3,2,1,4,9,1,23,3,3,3];

    println!("Numbers: {:?}", nums);
    println!("Mean: {}",mean(&nums));
    println!("median: {}",median(&mut nums));
    println!("mode: {}",mode(&nums));

    let string = "first".to_string();

    println!("{}'s pig latin is: {}", string, pig_latin(&string));

    let string = "apple".to_string();
    println!("{}'s pig latin is: {}", string, pig_latin(&string));
    

    let mut v: Vec<i32> = vec![1, 2, 3, 4];
    //prints 4
    println!("v's capacity is {}", v.capacity());
    println!("Address of v's first element: {:p}", &v[0]);//{:p} prints the address
    v.push(5);
    //prints 8
    println!("v's capacity is {}", v.capacity());
    println!("Address of v's first element: {:p}", &v[0]);

    //use crate::org::org::Organization;
    let mut my_org = org::Organization::initialize();
    my_org.add_employee("R&D", "Lee");
    my_org.add_employee("R&D", "Bruce");
    my_org.add_employee("R&D", "Bob");
    my_org.add_employee("Sales", "Nance");
    my_org.add_employee("Sales", "Jordan");
    my_org.add_employee("Sales", "Balky");
    my_org.list_employee("R&D");
    my_org.list_all();

}

#[allow(unused_mut, unused_variables)]
fn mean(nums:&Vec<i32>) -> i32 {

    let mut ans = 0;

    for n in nums {
        ans += n;
    }

    ans / nums.len() as i32
}

#[allow(unused_mut, unused_variables)]
fn median(nums:&mut Vec<i32>) -> i32 {

    nums.sort_by(|a, b| a.cmp(b));

    if let Some(n) = nums.get(nums.len()/2) {
        return *n // derefer
    }

    0

}

#[allow(unused_mut, unused_variables)]
fn mode (nums:&Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut map: HashMap<&i32, i32> = HashMap::new();

    for n in nums {
        let count = map.entry(n).or_insert(0);
        *count += 1;
    }

    let mut max: (&i32, i32) = (&0, 0); // 出现最多的

    for (key, value) in &map {
        println!("number: {}, occur: {}", key, value);
        if value > &max.1 {
            max.0 = key;
            max.1 = *value;
        }
    }

    *max.0
}

#[allow(unused_mut, unused_variables)]
fn pig_latin(s: &String) -> String {

    let mut ans = String::new();
    let first = s.chars().next();

    if let Some(c) = first {

        if is_consonant(c) {
            ans.push_str(&s[1..]);  
            ans.push_str("-" );  
            ans.push_str(&s[0..1]);  
            ans.push_str("ay");  

            return ans
        } else {
            ans.push_str(s);
            ans.push_str("-hay");
            return ans
        }
    
    }
    "".to_string()
}

#[allow(unused_mut, unused_variables)]
fn is_consonant(c: char) -> bool {
    match c {
        'a' => false,
        'e' => false,
        'i' => false,
        'o' => false,
        'u' => false,
        _ => true
    }
}


