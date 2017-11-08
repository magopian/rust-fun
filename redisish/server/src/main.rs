/// Server accepts:
/// - Get - Gives the OLDEST entry
/// - Put - Places the entry onto the vector
///
/// Useful types:
/// - `std::net::TcpListener` (and the `incoming()`)
/// - `std::collection::Vec` or `std::collections::VecDeque`

extern crate bufstream;

use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, Write};
use std::collections::VecDeque;
use bufstream::BufStream;

#[derive(Debug)]
struct Redisish {
    messages: VecDeque<String>,
}

impl Redisish {
    fn handle_client(&mut self, mut stream: TcpStream) {
        let mut stream = BufStream::new(stream);
        let mut content = String::new();
        stream.read_line(&mut content);

        if content == String::from("GET\n") {
            self.get(stream);
        } else if content.starts_with("PUT ") {
            let splitted: Vec<&str> = content.trim().split(" ").collect();
            if splitted.len() >= 2 {
                println!("Putting: {}", splitted[1]);
                self.messages.push_front(splitted[1].to_string());
            } else {
                println!("Malformed PUT");
            }
        } else {
            println!("Neither GETing nor PUTing...");
        }

        println!("New queue length: {}", self.messages.len());
    }

    fn get(&mut self, mut stream: BufStream<TcpStream>) {
        match self.messages.pop_back() {
            Some(message) => {
                println!("popped: {}", message);
                stream.write(message.as_bytes());
            },
            None => {
                println!("Nothing left!");
                stream.write(String::from("Nothing left!").as_bytes());
            }
        }
        stream.write(String::from("\n").as_bytes());
        stream.flush();
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    let mut messages = VecDeque::new();
    messages.push_front(String::from("foobar"));
    let mut redisish = Redisish { messages : messages};
    println!("Started the redis server");

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => redisish.handle_client(stream),
            Err(e) => println!("Error: {:?}", e)
        }
    }
}

#[test]
fn put() {
    assert!(false);
}
