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
