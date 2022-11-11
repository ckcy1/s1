use std::net::TcpListener;
fn main() {
    let listener=TcpListener::bind("127.0.0.1:3003").unwrap();
    println!("Running on port 3000...");
    for stream in listener.incoming() {
        let _stream =stream.unwrap();
        print!("Connection established!");
    }
}
