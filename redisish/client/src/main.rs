extern crate bufstream;

use std::net::TcpStream;
use std::io::{BufRead, Write, stdin};
use bufstream::BufStream;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:8000").expect("Couldn't connect to server");
    let mut stream = BufStream::new(stream);

    loop {
        println!("Enter command: either GET or PUT <string>");
        let mut command = String::new();
        stdin().read_line(&mut command).expect(
            "Failed to read line",
        );

        let response = send_and_receive(&mut stream, command).expect("IO error");
        if response.len() == 0 {
            println!("Got nothing from the server, disconnected?");
            break;
        }
        println!("Response: {:?}", response);
    }
}

fn send_and_receive(
    stream: &mut BufStream<TcpStream>,
    command: String,
) -> Result<String, std::io::Error> {
    stream.write(command.as_bytes())?;
    stream.flush()?;

    let mut response = String::new();
    stream.read_line(&mut response)?;
    Ok(response.trim().into())
}
