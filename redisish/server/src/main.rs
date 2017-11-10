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
/// x use multile modules (multiple files preferably to learn import semantics better)
/// x use an enum type for messages
/// - make it threaded
/// - multiple (named) channels (hashmap? `std::collections::hash_map::Entry`)
/// x remove all `unwrap` calls
/// - use the `?` operator with your own error type (enum is fine for now)
/// x use `&'a str` for your message parsing function (or: get rid of `String::from` where
/// possible)
/// - try deserializing and serializing with a library (serde_json)

extern crate bufstream;
mod redisish;

use std::net::{TcpListener};
use std::collections::VecDeque;
use std::thread;
use std::sync::mpsc::{Receiver, Sender, channel};
use redisish::{Command, Message, Redisish, handle_client};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Couldn't open listening socket");

    let mut messages = VecDeque::new();
    messages.push_front(String::from("foobar"));
    let mut redisish = Redisish::new(messages);
    println!("Started the redis server");

    let (tx, rx): (Sender<Message>, Receiver<Message>) = channel();
    // Spawning the receiver (handles the "database")
    thread::spawn(move || {
        loop {
            let message = rx.recv().expect("Couldn't receive from channel");
            println!("Message received: {:?}", message.command);
            match message.command {
                Command::INVALID =>  {
                    println!("Neither GETing nor PUTing...");
                    message.answer.send("Command invalid".into()).expect("Couldn't write answer to channel");
                },
                Command::GET => {
                    let data = redisish.get();
                    message.answer.send(data).expect("Couldn't write answer to channel");
                },
                Command::PUT(data) => {
                    println!("Putting: {}", data);
                    redisish.messages.push_front(data.into());
                    message.answer.send("ACK".into()).expect("Couldn't write answer to channel");
                    println!("New queue length: {}", redisish.messages.len());
                }
            };
        }
    });

    // accept connections and process them in separate threads
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client");
                let tx = tx.clone();
                thread::spawn(move || {
                    handle_client(tx, stream).expect("IO error");
                });
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
