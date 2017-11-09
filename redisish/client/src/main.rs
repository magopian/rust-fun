extern crate bufstream;

use std::net::TcpStream;
use std::io::{BufRead, Write, stdin};
use bufstream::BufStream;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:8000").unwrap();
    let mut stream = BufStream::new(stream);

    loop {
        println!("Enter command: either GET or PUT <string>");
        let mut command = String::new();
        stdin().read_line(&mut command).expect(
            "Failed to read line",
        );

        stream.write(command.as_bytes());
        stream.write(String::from("\n").as_bytes());
        stream.flush();

        let mut response = String::new();
        stream.read_line(&mut response);
        println!("Response: {:?}", response.trim());
    }
}
