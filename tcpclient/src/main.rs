use std::net::TcpStream;
fn main() {
    let _stream = TcpStream::connect("localhost:3008").unwrap();
    println!("Hello, world!");
}
