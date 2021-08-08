

fn main() {

    // Listing 18-1: Mixing if let, else if, else if let, and else
    let favorite_color: Option<&str> = None;
    let is_tuesday = true;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // Listing 18-2: Using a while let loop to print values for as long as stack.pop() returns Some
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // Listing 18-3: Using a pattern in a for loop to destructure a tuple
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // Listing 18-4: Using a pattern to destructure a tuple and create three variables at once
    let (x, y, z) = (1, 2, 3);

    // Listing 18-7: A function with parameters that destructure a tuple
    let mut point = (3, String::from("56"));
    print_coordinates(&mut point);
    println!("{:?}",point);

    
    let mut point = (3, String::from("56"));
    print_coordinates2(point);
    //println!("{:?}",point);

}
// Listing 18-7: A function with parameters that destructure a tuple

// 参数中的 String 不能 move (因为没有实现trait), 所以只能引用. 
// x: &mut i32, y: &mut String
fn print_coordinates((x, y): &mut (i32, String)) {
    y.push_str("rooms");    // 可以直接调用方法
    (*y).push_str("rooms"); // 也可以解引用后调用(通过方法修改值)
    *y = "1".to_string();   // 修改参数变量值(直接修改)
    *x = 2;                 // 修改参数变量值(直接修改)
    println!("Current location: ({}, {})", x, y); // 
}

// Listing 18-7: A function with parameters that destructure a tuple
fn print_coordinates2(mut t: (i32, String)) {
    // t 的类型是 mut (i32, String), 参数 tuple move 到了这里.
    t.0 = 2;               // 修改值(直接修改)
    t.1 = "1".to_string(); // 修改值(直接修改)
    t.1.push_str("123");   // 修改值(通过方法)
    println!("Current location: ({}, {})", t.0, t.1);
}

// 试图 move 类型 String, 可以
fn print_coordinates3( (x, y):  (i32, String)) {
    
    println!("Current location: ({}, {})", x, y);
}

// // 试图 move 类型 String, 因为 String 没有 Copy 功能, 然而 String 在一个引用中,
// // 所以无法 move.
// fn print_coordinates4( &mut (x, y):  &mut (i32, String)) {
//     // x.dd();
//     println!("Current location: ({}, {})", x, y);
// }

fn print_coordinates5(&(x, y): &(i32, i32)) { // 
    // x.dfd();
    println!("Current location: ({}, {})", x, y);
}

