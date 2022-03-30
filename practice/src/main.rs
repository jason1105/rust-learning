#[derive(Debug)]
struct MyBox {
    name: String,
}

fn main() {
    let a = String::from("abcd");
    for (i, s) in a.as_bytes().iter().rev().enumerate() {
        println!("a[{}] = {}", i, s);
    }

    let numbers = [1, 2, 3, 4, 5];

    let zero = "0".to_string();

    let result = numbers
        .iter()
        .fold(zero, |acc, &x| format!("({} + {})", acc, x));
    assert_eq!(result, "(((((0 + 1) + 2) + 3) + 4) + 5)");

    let mut my_box = MyBox {
        name: "my_box".to_string(),
    };
    println!("{:?}", &my_box); // MyBox { name: "my_box" }

    let name = &mut my_box.name;
    *name = "your_box".to_string();
    println!("{:?}", &my_box); // MyBox { name: "your_box" }

    my_box.name = "our_box".to_string();
    println!("{:?}", &my_box); // MyBox { name: "our_box" }

    let a = String::from("111");
    let a = Vec::from(a);
    let b = String::from("111");
    let b = Vec::from(b);
    println!("{}", a[0] - b'0' + b[1]);
}
mod test {
    use std::{pin::Pin, task::Poll};

    use futures_io::{AsyncBufRead, AsyncRead};

    //不能通过编译
    struct Body(Vec<u8>);
    impl AsyncRead for Body {
        fn poll_read(
            self: Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
            buf: &mut [u8],
        ) -> Poll<futures_io::Result<usize>> {
            todo!()
        }
    }

    impl AsyncBufRead for Body {
        fn poll_fill_buf(
            self: Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> Poll<futures_io::Result<&[u8]>> {
            // Poll::Ready(Ok(&self.0))
            Poll::Ready(Ok(&(Pin::into_inner(self).0)))
        }

        fn consume(self: Pin<&mut Self>, amt: usize) {
            todo!()
        }
    }
}
