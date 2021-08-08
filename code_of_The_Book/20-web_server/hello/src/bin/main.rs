use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::sync::mpsc;
use std::time;

fn handle_client(mut stream: TcpStream) {

    let mut data = [0; 1024];

    stream.read(&mut data).unwrap();

    // println!("Server: {}", String::from_utf8_lossy(&data));

    let get = b"GET / HTTP/1.1\r\n";
    let get_sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status, res_filename) = if data.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if data.starts_with(get_sleep) {
        thread::sleep(time::Duration::from_secs(2));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 4.4 NOT FOUND", "404.html")
    };

    let content = fs::read_to_string(res_filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        content.len(),
        content
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

use hello::ThreadPool;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    let pool = ThreadPool::new(4);

    // accept connections and process them serially
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_client(stream);
        });
    }

    Ok(())
}