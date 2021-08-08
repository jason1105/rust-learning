
// Listing 15-20: A library to keep track of how close a value is to a maximum value and warn when the value is at certain levels
#[allow(unused_variables, unused_mut, unused_assignments)]
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[allow(unused_variables, unused_mut, unused_assignments)]
#[cfg(test)]
mod tests {

    // Listing 15-22: Using RefCell<T> to mutate an inner value while the outer value is considered immutable
    use super::Messenger;
    use super::LimitTracker;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> Self {
            MockMessenger {
                sent_messages: RefCell::new(vec![])
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(msg.to_string());
        }
    }

    // Listing 15-23: Creating two mutable references in the same scope to see that RefCell<T> will panic
    // WILL CAUSE PANIC!!!
    // impl Messenger for MockMessenger {
    //     fn send(&self, message: &str) {
    //         let mut one_borrow = self.sent_messages.borrow_mut();
    //         let mut two_borrow = self.sent_messages.borrow_mut();

    //         one_borrow.push(String::from(message));
    //         two_borrow.push(String::from(message));
    //     }
    // }

    #[test]
    fn test_mock() {
        let mut mock_sender = MockMessenger::new();
        let mut tracker = LimitTracker::new(&mock_sender, 10);
        
        tracker.set_value(10);
        assert_eq!(1, mock_sender.sent_messages.borrow().len());
        assert_eq!(vec!["Error: You are over your quota!"], *mock_sender.sent_messages.borrow());
        
        tracker.set_value(9);
        assert_eq!(2, mock_sender.sent_messages.borrow().len());
        assert_eq!("Urgent warning: You've used up over 90% of your quota!", mock_sender.sent_messages.borrow()[1]);
        
        // tracker.set_value(8);
        // assert_eq!(vec!["Warning: You've used up over 75% of your quota!"], mock_sender.sent_messages);
    }

    #[test]
    fn test_ref() {
        let mut i = String::from("5");
        let mut j = String::from("10");
        
        let mut i = 5;
        let mut j = &mut i;
        *j += 1;
        
        assert_eq!(6, *j);
        println!("i: {}", i);
        assert_eq!(6, i);
    }

    #[test]
    fn test_15_24() {

        #[derive(Debug)]
        enum List {
            Cons(Rc<RefCell<i32>>, Rc<List>),
            Nil,
        }

        use List::{Cons, Nil};
        use std::cell::RefCell;
        use std::rc::Rc;

        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

        *value.borrow_mut() += 10;  // which uses the automatic dereferencing feature we discussed in Chapter 5 (see the section “Where’s the -> Operator?”) to dereference the Rc<T> to the inner RefCell<T> value.

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);


    }

    #[test]
    fn test333() {

        /*
        let mut y = String::from("1");
        y 的数据类型(指向的内存区域的类型)是 String, 
        y 是资源 String::from("1") 的所有者(owner), 
        在编译期, y 被替换为真正保存了 "1" 这个数据的内存地址. 例如: 0x09e3a317
        因为使用了 mut, 所以 y 区域的内容可以修改, 
        换句话说, 因为使用了 mut, 所以 "1" 所在的内存区域是可以被编辑的.
        总结: 申请了一块可以编辑的内存标记为 y, 里面存放了 String::from("1")
        */
        let mut y = String::from("1");  // 注意, y 是 owner, 决定了什么时候可以释放内存. 即 0x09e3a317 地址一旦离开作用域, 将被回收.
        y = String::from("11");         // 编辑内存中的内容
        println!("y: {}", y);   // y: 11
        println!();
        
        /*
        & 在 Rust 中叫 borrow, 暂时借用, 并不是变量的主人, 所以不拥有变量. 
        & 可以理解为 c语言 中的取地址操作, 例如:
        &y 的意思是: 借用一下 y. 那怎么借用呢? 我只要 y 的地址, 通过 y 和数据打交道, 所以 y 还是数据的owner, 我只是借用一下,
        &mut y 的意思是, 借用一下, 同时还有修改数据的权限
        例如:
        let a = &y; 的意思是, 申请了一块只读内存, 标记为a, 里面存放的是 y 的地址(且不能修改成别的地址). a 只可以使用但不能修改 y (这个 a 不就是指针嘛)
        let a = &mut y; 的意思是, 申请了一块只读内存, 标记为a, 里面存放的是 y 的地址(且不能修改成别的地址). a 有权修改 y 的内容( a 是一个可变指针) 
        let mut a = &y; 的意思是, 申请了一块可读写内存, 标记为a, 里面存放的是 y 的地址(也可以修改成存放 z 的地址). a 只可以使用但不能修改 y (这个 a 不就是指针嘛)
        let mut a = &mut y; 的意思是, 申请了一块可读写内存, 标记为a, 里面存放的是 y 的地址(也可以修改成存放 z 的地址), a 有权修改 y 的内容( a 是一个可变指针) 

        borrow 有两个规则:
        1. 同一时刻, 一个资源只能有一个 &mut borrow 或者多个 & borrow, 
        2. 引用必须是有效的. 即不能引用一个不存在的东西.
        
        borrow 的两个规则可以可以理解为读写锁, 
            在一个作用域(事务)当中, 对资源进行锁定以避免竞争条件, 要么都是读锁(s), 要么只有一个写锁(x).
            & 理解为读锁.
            &mut 理解为写锁.
        这样设计的好处: 避免一段代码中不同的逻辑之间的干扰(类似事务之间的干扰)
        隐含的 borrow: 在方法参数中, 例如 println!()
        */
        {
            println!("-------------- Block A");
            let a = &mut y;             // a 的数据类型(指向的内存区域的类型)是 &mut String, 通过 mut 引用 y (拥有了 y 的 x 锁), 
            // let mut c = &mut y;      // c 也想要 y 的 x 锁, 这是不可能的, y 上只能加一把 x 锁. 已经被 a 占用了, 且 a 还没有释放.
            // y = String::from("111"); // owner 当然有最大权限, 但此时因为 y 上有一把 x 锁被 a 占用, 且 a 还没有释放, 所以 owner 也无权修改.
            // println!("y: {}", y);       // 这里的 y 作为了方法参数, 是 & borrow (相当于 s 锁), 因为 y 上有 x 锁, 这个 borrow 是不被允许的.
            *a = String::from("2");     // a 拥有 x 锁, 可以写 y. 此后 a 作用域结束, 释放了 x 锁.
            println!("y: {}", y);       // 现在 y 上没有任何锁, y: 2
            y = String::from("111");    // 因为 a 的作用域已经结束, y 上没有锁, 所以现在 owner 可以修改了.
            let mut c = &mut y;         // y 上没有锁, c 现在可以加一把 x 锁
            //println!("y: {}", y);     // y 上有 c 加的锁, 且 c 还没有释放, 所以不能再 borrow
            *c = String::from("3");     // c 修改了 y, 此后 c 作用域结束, 释放 x 锁.
            println!("y: {}", y);       // 现在 y 上没有任何锁, y: 3
            println!();
        } 

        {
            println!("-------------- Block B");
            let a = & y;                // a 拥有了 y 的 s 锁
            // let c = &mut y;          // c 想要 y 的 x 锁, 这是不可能的, 因为 s 和 x 不能共存, a 已经持有 s 锁且 a 还没有释放, 所以无法加 x 锁.
            // y = String::from("111"); // owner 当然有最大权限, 但此时因为 y 上有一把 s 锁, 且还未释放, 所以 owner 也无权修改.
            let c = & y;                // 但 c 可以得到 y 的 s 锁, 共享锁可以加多次, 不多 c 并没有使用, 所以随即这把 s 锁就失效了
            println!("y: {}", y);       // 这里的 borrow 相当于 s 锁, 因为 s 锁可以加多次, 所以没有问题. y: 3
            println!("a: {}", a);       // 此后 a 作用域结束, 释放了 s 锁. a: 3
            y = String::from("111");    // y 上没有任何锁, y 是owner, 当然可以修改
            println!("y: {}", y);       // y: 111

            println!();
        }
        

        // let mut c = String::from("3");
        // a = &mut c;
        // *a = String::from("4");
    }

    // Listing 15-25: A cons list definition that holds a RefCell<T> so we can modify what a Cons variant is referring to
    #[test]
    fn test15_25() {
        use std::rc::Rc;
        use std::cell::RefCell;

        #[derive(Debug)]
        enum List {
            Cons(i32, RefCell<Rc<List>>),
            Nil,
        }

        impl List {
            fn tail(&self) -> Option<&RefCell<Rc<List>>> {
                if let Cons(_, list) = self {
                    Some(list)
                } else {
                    None
                }
            }
        }

        use List::{Cons, Nil};

        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));   // a
        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a)))); // b -> a

        println!("b.next: {:?}", b.tail());

        if let Some(list) = a.tail() {
            *list.borrow_mut() = Rc::clone(&b); // a -> b
        }

        println!("a count: {}", Rc::strong_count(&a));
        println!("b count: {}", Rc::strong_count(&b));
        //println!("a.next: {:?}", a.tail()); // cause error for over stack

    }

    // Listing 15-27: Creating a leaf node with no children and a branch node with leaf as one of its children
    // Listing 15-28: A leaf node with a weak reference to its parent node branch
    #[test]
    fn test99_25() {
        use std::rc::Rc;
        use std::rc::Weak;
        use std::cell::RefCell;
        
        #[derive(Debug)]
        struct Node {
            value: u32,
            children: RefCell<Vec<Rc<Node>>>,
            parent: RefCell<Weak<Node>>
        }

        let leaf = Rc::new(Node{
            value: 2,
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
        });

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        let root = Node{
            value: 1,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            parent: RefCell::new(Weak::new()),
        };
        let root = Rc::new(root);
        
        println!("root children = {:?}", root.children.borrow());
        println!(
            "root strong = {}, weak = {}",
            Rc::strong_count(&root),
            Rc::weak_count(&root),
        );
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );


        *leaf.parent.borrow_mut() = Rc::downgrade(&root);

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "root strong = {}, weak = {}",
            Rc::strong_count(&root),
            Rc::weak_count(&root),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        println!("tree = {:?}", root);
        /*
        Node { 
            value: 1, 
            children: RefCell { 
                        value: [
                            Node { 
                                value: 2, 
                                children: RefCell { value: [] }, 
                                parent: RefCell { value: (Weak) } 
                            }
                        ]
                    }, 
            parent: RefCell { value: (Weak) } 
        }
        */

    }
}