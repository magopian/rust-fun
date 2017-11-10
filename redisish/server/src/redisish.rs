use std::net::TcpStream;
use std::io::{BufRead, Error, Write};
use std::collections::VecDeque;
use std::sync::mpsc::{Receiver, Sender, channel};
use bufstream::BufStream;

pub struct Message {
    pub command: Command,
    pub answer: Sender<String>,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    GET,
    PUT(String),
    INVALID,
}

impl Command {
    fn parse(command: &str) -> Self {
        let command = command.trim();
        if command == "GET" {
            Command::GET
        } else if command.starts_with("PUT ") {
            if command.len() < 4 {
                Command::INVALID
            } else {
                let (_, data) = command.trim().split_at(4);
                Command::PUT(data.into())
            }
        } else {
            Command::INVALID
        }
    }
}

#[derive(Debug)]
pub struct Redisish {
    pub messages: VecDeque<String>,
}

impl Redisish {
    pub fn new(messages: VecDeque<String>) -> Self {
        Self { messages: messages }
    }

    pub fn get(&mut self) -> String {
        match self.messages.pop_back() {
            Some(message) => {
                println!("popped: {}", message);
                message
            }
            None => {
                println!("Nothing left!");
                "Nothing left".into()
            }
        }
    }
}

pub fn handle_client(chan: Sender<Message>, stream: TcpStream) -> Result<(), Error> {
    let mut buffered = BufStream::new(stream);

    loop {
        let mut content = String::new();
        let content_len = buffered.read_line(&mut content);
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

        let command = Command::parse(&content);
        let (tx, rx): (Sender<String>, Receiver<String>) = channel();
        chan.send(Message {
            command: command,
            answer: tx,
        }).expect("Couldn't send message to channel");
        let response = rx.recv().expect("Couldn't read response from channel");
        write!(buffered, "{}\n", response).and_then(
            |_| buffered.flush(),
        )?;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_parse_get() {
        assert_eq!(Command::parse("GET\n"), Command::GET);
    }

    #[test]
    fn test_command_parse_put() {
        assert_eq!(
            Command::parse("PUT foo bar\n"),
            Command::PUT("foo bar".into())
        );
    }

    #[test]
    fn test_command_parse_put_invalid() {
        assert_eq!(Command::parse("PUT "), Command::INVALID);
    }

    #[test]
    fn test_command_parse_invalid() {
        assert_eq!(Command::parse("foo"), Command::INVALID);
    }
}
