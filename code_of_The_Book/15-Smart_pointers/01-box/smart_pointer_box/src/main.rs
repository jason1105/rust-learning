// https://doc.rust-lang.org/book/ch15-02-deref.html

// Listing 15-1: Storing an i32 value on the heap using a box
#[allow(unused_variables, unused_mut, unused_assignments)]
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    // Listing 15-2: The first attempt at defining an enum to represent a cons list data structure of i32 values
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    use List::{Cons, Nil};

    // Listing 15-3: Using the List enum to store the list 1, 2, 3
    let list = Cons(1,Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));


    // Listing 15-6: Using the dereference operator to follow a reference to an i32 value
    let x = 5;
    let y = &x; // y is a pointer

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x); // y is a pointer

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Listing 15-8: Defining a MyBox<T> type
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        pub fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    // Listing 15-10: Implementing Deref on MyBox<T>
    use std::ops::Deref;

    impl<T> Deref for MyBox<T> {
        type Target = T;  // Associated type covered in Chapter 19

        fn deref(&self) -> &Self::Target { // Self::Target means Deref::Target, the type of return value must be a reference. so add &.
            &self.0 // we need to return value of type of reference, so add &
        }
    }

    let x = 5;
    let y = MyBox::new(x); // y is a pointer

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Listing 15-11: A hello function that has the parameter name of type &str
    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }

    // Listing 15-12: Calling hello with a reference to a MyBox<String> value, which works because of deref coercion
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);

    // Listing 15-14: A CustomSmartPointer struct that implements the Drop trait where we would put our cleanup code
    struct CustomSmartPointer {
        data: String,
    }
    
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    drop(c); // we can manually call drop method to deallocate memory otherwise c will be cleared automatically when it out of scope at the end of the method

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");


}

#[allow(unused_variables, unused_mut, unused_assignments)]
#[cfg(test)]
mod tests {
        // Listing 15-17: Demonstrating we’re not allowed to have two lists using Box<T> that try to share ownership of a third list
        // enum List {
        //     Cons(i32, Box<List>),
        //     Nil,
        // }
        // use List::{Cons, Nil};
        // fn test1() {
        //     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
        //     let b = Cons(3, Box::new(a));
        //     let c = Cons(4, Box::new(a));
        // }

        // Listing 15-18: A definition of List that uses Rc<T>

        use std::rc::Rc;
    
        enum List {
            Cons(i32, Rc<List>),
            Nil,
        }
        use List::{Cons, Nil};

        #[test]
        fn test2() {
            let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
            let b = Cons(3, Rc::clone(&a));
            let c = Cons(4, Rc::clone(&a));

        }

        // Listing 15-19: Printing the reference count
        #[test]
        fn test15_19() {
            let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
            println!("count after creating a = {}", Rc::strong_count(&a));
            let b = Cons(3, Rc::clone(&a));
            println!("count after creating b = {}", Rc::strong_count(&a));
            {
                let c = Cons(4, Rc::clone(&a));
                println!("count after creating c = {}", Rc::strong_count(&a));
            }
            println!("count after c goes out of scope = {}", Rc::strong_count(&a));
        }

        // test3 和 test4 比较一下
        #[test]
        fn test3 () {
            use std::rc::Rc;

            #[derive(Debug)]
            struct Student {
                name: String,
                class: Rc<String> // 使用 Rc 引用
            }
            
            let class = Rc::new(String::from("math"));
            let lee = Student{
                    name: "lee".to_string(),
                    class: Rc::clone(&class)
                };
            let bob = Student {
                    name: "bob".to_string(),
                    class: Rc::clone(&class)
                };

            another(class.clone()); // Rc 的 clone 指的是克隆所有权的意思.

            println!("lee: {:?}", lee);

            fn another(class: Rc<String>) {
    
            }

        }

        #[test]
        fn test4() {
            #[derive(Debug)]
            struct Student<'a> {
                name: String,
                class: &'a String // 没使用Rc, 使用普通的引用
            }

            fn new_student(name: String, class: &String) -> Student {
                Student {
                    name: name,
                    class: class
                }
            }

            let class = String::from("math");
            let lee = new_student("lee".to_string(), &class);
            let bob = new_student("bob".to_string(), &class);
            // another(class); // class 一旦被销毁, 下面的 lee 便无法引用了.
            println!("lee: {:?}", lee);

            #[allow(dead_code)]
            fn another(class: String) {
    
            }
        }


}