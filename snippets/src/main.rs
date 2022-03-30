#[macro_export]
macro_rules! syntax_struct {
    (
        struct $Name:ident {
            keywords: {
                /*
                $ () 表示一个匹配
                , 表示可选的字面量
                * 表示匹配0次或多次
                $color:expr 匹配一个表达式, 且定义为 $color 变量
                $words:expr 匹配一个表达式, 且定义为 $words 变量
                [ 和 ] 以及 ; 都是字面量
                */
                $([$color:expr; $($words:expr),*]),*
            }
        }
    ) => {
        /* Expand macro */
        struct $Name {}

        impl $Name{
            fn show() {
                $(
                    println!("{:?}", $color); // $color 所处的深度必须与匹配模式中的深度保持一致
                    $(
                        print!("{},", $words); // $words 所处的深度必须与匹配模式中的深度保持一致
                    )*
                )*
            }
        }
    };
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

syntax_struct! (
    struct S {
        /* 这里的内容更加灵活, 不用满足语法要求, 其中的内容在宏内部使用 $expr 接收 */
        keywords: {[Color::Red; "1", "2"]} //
    }
);

//
pub fn main() {
    S::show();
    println!("");

    // Macro test1
    let abcde = 123;
    test1!(abcde);
    // test1!(u32); // Error, because test1 restrict the parameter type must be a value.

    // Macro test234
    test234!(u32, String);
    // test234!(abcde); // Error, because test234 restrict the parameter type must be a type in build.
}

#[macro_export]
macro_rules! test1 {
    /*
    Identifier is including names of functions,
    variables, parameters, struct fields, modules,
    crates, constants, macros, static values, attributes,
    types, traits, or lifetimes.
    */
    ($( $i:ident ),*) => {
        $(
            println!("{}", $i);
        )*
    };
}

#[macro_export]
macro_rules! test234 {
    /*
    参数是 identifier, 可能是函数名称, 变量名, 参数, 结构体名, 模块名称, 等等. 但现在并不知道是什么.
    转换参数的格式, 然后调用另一个宏
    */
    ($( $i:ident ),*) => { test567!($( $i );*) }
}

#[macro_export]
macro_rules! test567 {
    ($( $i:ident );*) => {
        $(
            /*
            由于这里使用了 $i, 且是作为类型来使用的.
            所以, 调用这个 macro , 就必须传递类型.
            也就是说, test234 必须传递一个或多个类型.
            也就是说, 在 main 方法中调用 test234 时, 虽然 test234 只要求是 ident, 但却必须传递一个或多个类型.
            */
            let v:$i = $i::default();

            println!("{:?}", v);
        )*
    };
}
