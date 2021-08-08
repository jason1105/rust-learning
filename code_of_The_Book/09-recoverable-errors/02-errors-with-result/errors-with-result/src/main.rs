/* 
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
*/

use std::fs::File;
use std::io;
use std::fs;
use std::io::Read;

fn main() {

    
    fs::remove_file("hello.txt");

    // let f = File::open("hello.txt").unwrap(); // 直接拆包
    // let f = File::open("hello.txt").expect("Failed to open hello.txt"); // 拆包, 如果是错误则打印自定义的错误消息.

    // Listing 9-3: Opening a file
    // let f = File::open("hello.txt"); // Result

    // Listing 9-4: Using a match expression to handle the Result variants that might be returned
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // Listing 9-5: Handling different kinds of errors in different ways
    // let f = File::open("hello.txt"); 
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() { // error is type of io::Error which has method of kind()
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };

    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     match error.kind() { // error is type of io::Error which has method of kind()
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     }
    // });

    // let f = File::open("hello.txt").unwrap(); // no such file.

    // Listing 9-6: A function that returns errors to the calling code using match
    let name = read_username_from_file().unwrap_or_else(|error| {
        // 错误处理
        // panic!("Problem opening the file: {:?}", error);
        "".to_string()
    });
}

// Listing 9-6: A function that returns errors to the calling code using match
fn read_username_from_file() -> Result<String, io::Error> { // 定义可以包含特定错误信息的接口
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // 把错误交给调用者处理
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e), // 把错误交给调用者处理
    }
}

// Listing 9-7: A function that returns errors to the calling code using the ? operator
// 这个方法和上面的方法功能一样, 但使用了 ? , 简化了代码量
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // ? 表示前面的 Result 如果是 Err 则函数直接返回.
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Listing 9-8: Chaining method calls after the ? operator
// 这个方法和上面的一样, 但使用了链式调用, 更加简洁.
fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// Listing 9-9: Using fs::read_to_string instead of opening and then reading the file
// 这个方法和上面的一样, 更简单.
fn read_username_from_file_4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// fn main() -> Result<(), Box<dyn Error>> {
//     let f = File::open("hello.txt")?;

//     Ok(())
// }