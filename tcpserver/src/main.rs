use std::net::TcpListener;
fn main() {
    let listener=TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000...");
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("connection established")// 不要敲成print!("Connection established!")终端显示不了的
    }
}

