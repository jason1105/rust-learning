#[derive(Default, Clone, Copy)]
struct S {
    x: i32, // 因为 i32 是 Copy 的，所以 S 可以 derive Copy
}

#[derive(Default)]
struct SS {
    o: String, // 因为 String 没有实现 Copy, 所以 SS 也不可以 Copy
}

pub fn main() {
    /*
    关于变量 move, move 可以在函数传递时发生, 在赋值时发生,
    */
    let s = S::default();
    s; // copy
    let _f = s.x; // OK: 没有问题, 因为 s.x 是 Copy 的
    println!("{}", s.x);

    let ss = SS::default();
    ss; // 场合一: move, 虽然没有接收者, 但这是变量赋值的场景, 下面的 MIR 中可以看到一些细节.

    // let _f = ss.o; // 场合二: 部分move

    // println!("{}", ss.o); // Error, borrowed of moved value
}

/*

====Source code===
#[derive(Default)]
struct SS {
    o: Option<String>, // 因为 String 没有实现 Copy, 所以 SS 也不可以 Copy
}

pub fn main() {

    let ss = SS::default();
    ss; // 发生move
}


======MIR======

// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
fn <impl at src/main.rs:1:10: 1:17>::default() -> SS {
    let mut _0: SS;                      // return place in scope 0 at src/main.rs:1:10: 1:17
    let mut _1: std::option::Option<std::string::String>; // in scope 0 at src/main.rs:3:5: 3:22

    bb0: {
        _1 = <Option<String> as Default>::default() -> bb1; // scope 0 at src/main.rs:3:5: 3:22
                                         // mir::Constant
                                         // + span: src/main.rs:3:5: 3:22
                                         // + literal: Const { ty: fn() -> std::option::Option<std::string::String> {<std::option::Option<std::string::String> as std::default::Default>::default}, val: Value(Scalar(<ZST>)) }
    }

    bb1: {
        (_0.0: std::option::Option<std::string::String>) = move _1; // scope 0 at src/main.rs:1:10: 1:17
        return;                          // scope 0 at src/main.rs:1:17: 1:17
    }
}

fn main() -> () {
    let mut _0: ();                      // return place in scope 0 at src/main.rs:6:15: 6:15
    let _1: SS;                          // in scope 0 at src/main.rs:8:9: 8:11
    let _2: SS;                          // in scope 0 at src/main.rs:9:5: 9:7
    scope 1 {
        debug ss => _1;                  // in scope 1 at src/main.rs:8:9: 8:11
    }

    bb0: {
        _1 = <SS as Default>::default() -> bb1; // scope 0 at src/main.rs:8:14: 8:27
                                         // mir::Constant
                                         // + span: src/main.rs:8:14: 8:25
                                         // + literal: Const { ty: fn() -> SS {<SS as std::default::Default>::default}, val: Value(Scalar(<ZST>)) }
    }

    bb1: {
        _2 = move _1;                    // scope 1 at src/main.rs:9:5: 9:7  在这里发生move
        drop(_2) -> bb2;                 // scope 1 at src/main.rs:9:7: 9:8
    }

    bb2: {
        return;                          // scope 0 at src/main.rs:10:2: 10:2
    }
}

*/

/*
===Source Code===

#[derive(Default)]
struct SS {
    o: Option<String>, // 因为 String 没有实现 Copy, 所以 SS 也不可以 Copy
}

pub fn main() {

    let ss = SS::default();
    let s = ss; // 发生move
}


======MIR======

// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
fn <impl at src/main.rs:1:10: 1:17>::default() -> SS {
    let mut _0: SS;                      // return place in scope 0 at src/main.rs:1:10: 1:17
    let mut _1: std::option::Option<std::string::String>; // in scope 0 at src/main.rs:3:5: 3:22

    bb0: {
        _1 = <Option<String> as Default>::default() -> bb1; // scope 0 at src/main.rs:3:5: 3:22
                                         // mir::Constant
                                         // + span: src/main.rs:3:5: 3:22
                                         // + literal: Const { ty: fn() -> std::option::Option<std::string::String> {<std::option::Option<std::string::String> as std::default::Default>::default}, val: Value(Scalar(<ZST>)) }
    }

    bb1: {
        (_0.0: std::option::Option<std::string::String>) = move _1; // scope 0 at src/main.rs:1:10: 1:17
        return;                          // scope 0 at src/main.rs:1:17: 1:17
    }
}

fn main() -> () {
    let mut _0: ();                      // return place in scope 0 at src/main.rs:6:15: 6:15
    let _1: SS;                          // in scope 0 at src/main.rs:8:9: 8:11
    scope 1 {
        debug ss => _1;                  // in scope 1 at src/main.rs:8:9: 8:11
        let _2: SS;                      // in scope 1 at src/main.rs:9:9: 9:10
        scope 2 {
            debug s => _2;               // in scope 2 at src/main.rs:9:9: 9:10
        }
    }

    bb0: {
        _1 = <SS as Default>::default() -> bb1; // scope 0 at src/main.rs:8:14: 8:27
                                         // mir::Constant
                                         // + span: src/main.rs:8:14: 8:25
                                         // + literal: Const { ty: fn() -> SS {<SS as std::default::Default>::default}, val: Value(Scalar(<ZST>)) }
    }

    bb1: {
        _2 = move _1;                    // scope 1 at src/main.rs:9:13: 9:15
        drop(_2) -> bb2;                 // scope 1 at src/main.rs:10:1: 10:2
    }

    bb2: {
        return;                          // scope 0 at src/main.rs:10:2: 10:2
    }
}


*/
