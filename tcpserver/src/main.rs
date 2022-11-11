use std::net::TcpListener;
use std::io::{Read,Write};
fn main() {
    let listener=TcpListener::bind("127.0.0.1:3009").unwrap();
    println!("Running on port 3000...");
    println!("{:?}",listener);
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("connection established");// 不要敲成print!("Connection established!")终端显示不了的
         let mut buffer = [0;1024];
        println!("11{:?}",stream);
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}

