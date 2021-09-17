#[derive(Debug)]
struct MyBox {
    name: String,
}

fn main() {
    let mut my_box = MyBox {
        name: "my_box".to_string(),
    };
    println!("{:?}", &my_box); // MyBox { name: "my_box" }

    let name = &mut my_box.name;
    *name = "your_box".to_string();
    println!("{:?}", &my_box); // MyBox { name: "your_box" }

    my_box.name = "our_box".to_string();
    println!("{:?}", &my_box); // MyBox { name: "our_box" }
}
