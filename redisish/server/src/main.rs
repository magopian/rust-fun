/// Server accepts:
/// - Get - Gives the OLDEST entry
/// - Put - Places the entry onto the vector
///
/// Useful types:
/// - `std::net::TcpListener` (and the `incoming()`)
/// - `std::collection::Vec` or `std::collections::VecDeque`
///
/// Expanding on it:
/// - make what you can constants
/// - Add tests
/// - use multile modules (multiple files preferably to lean import semantics better)
/// - use an enum type for messages
/// - make it threaded
/// - multiple (named) channels (hashmap? `std::collections::hash_map::Entry`)
/// - remove all `unwrap` calls
/// - use the `?` operator with your own error type (enum is fine for now)
/// - use `&'a str` for your message parsing function (or: get rid of `String::from` where
/// possible)
/// - try deserializing and serializing with a library (serde_json)

extern crate bufstream;

use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, Write};
use std::collections::VecDeque;
use bufstream::BufStream;

enum Command {
    GET,
    PUT(String),
    INVALID,
}

impl Command {
    fn parse(command: &str) -> Self {
        if command == "GET\n" {
            Command::GET
        } else if command.starts_with("PUT ") {
            let (_, data) = command.trim().split_at(4);
            if data.len() != 0 {
                Command::PUT(data.into())
            } else {
                Command::INVALID
            }
        } else {
            Command::INVALID
        }
    }
}

#[derive(Debug)]
struct Redisish {
    messages: VecDeque<String>,
}

impl Redisish {
    fn handle_client(&mut self, stream: TcpStream) -> Result<(), std::io::Error> {
        let mut stream = BufStream::new(stream);

        loop {
            let mut content = String::new();
            let content_len = stream.read_line(&mut content);
            match content_len {
                Ok(len) => {
                    if len == 0 {
                        println!("Client disconnected, closing");
                        return Ok(());
                    } else {
                        println!("Content: {}", content);
                    }
                }
                Err(e) => return Err(e),
            };

            match Command::parse(&content) {
                Command::GET => self.get(&mut stream)?,
                Command::PUT(data) => {
                    println!("Putting: {}", data);
                    self.messages.push_front(data.into());
                    stream.write(String::from("ACK\n").as_bytes())?;
                }
                Command::INVALID => {
                    println!("Neither GETing nor PUTing...");
                    stream.write(
                        String::from(
                            "Couldn't recognize command, please use `GET` or `PUT <string>`\n",
                        ).as_bytes(),
                    )?;
                }
            }
            stream.flush()?;

            println!("New queue length: {}", self.messages.len());
        }
    }

    fn get(&mut self, stream: &mut BufStream<TcpStream>) -> Result<(), std::io::Error> {
        match self.messages.pop_back() {
            Some(message) => {
                println!("popped: {}", message);
                stream.write(message.as_bytes())?;
            }
            None => {
                println!("Nothing left!");
                stream.write(String::from("Nothing left!").as_bytes())?;
            }
        }
        stream.write(String::from("\n").as_bytes())?;
        Ok(())
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Couldn't open listening socket");

    let mut messages = VecDeque::new();
    messages.push_front(String::from("foobar"));
    let mut redisish = Redisish { messages: messages };
    println!("Started the redis server");

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client");
                redisish.handle_client(stream).expect("IO error");
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }
}

#[test]
fn put() {
    assert!(false);
}
