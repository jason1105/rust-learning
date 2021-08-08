fn main() {
    another_function(5);
    another_function2(2, 3);

    // let x = (let y = 6); // error on compiling
    // let x = y = 6; // compiling error
    /*
        Rust 中使用的最多的是表达式.
        let x = 6; 这里的 6 是一个表达式
        下面的都是表达式
        - 调用方法, 调用宏
        - {} 也是一个表达式.
    */

    let x = 5;

    // 这个语句块
    let y = {
        let x = 3;
        x + 1 // 注意, 语句块中的返回值不用加 ";", 如果加上了, 就变成语句块了.
    };

    println!("The value of y is: {}", y);

    println!("What 1 pulus 1 is: {}", plus_one(1));
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function2(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

/*
使用 -> 指定返回值类型
*/
fn five() -> i32 {
    // 可以提前返回
    // return 3

    //
    5 // 返回 5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
