use std::net::TcpStream;
fn main() {
    let _stream = TcpStream::connect("localhost:3003").unwrap();
    println!("Hello, world!");
}
