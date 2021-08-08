# 介绍

跟着 Rust 官方的 “[The Rust Programing Language](https://doc.rust-lang.org/book/title-page.html)” 中的 [第12章](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) 编写的一个小程序, 使用指定的关键字过滤文本文件内容, 并输出相关的行. 

# 使用

先[准备 Rust 环境](https://doc.rust-lang.org/book/ch01-01-installation.html)

然后下载代码

```
git clone https://github.com/jason1105/minigrep.git
```

尝试运行一下

```
cd minigrep
cargo run <query> <filename>
# 例如 cargo run to poem.txt
```

还可以编译成 exe

```bash
cargo build --release
```

