use std::net::TcpStream;
use std::io::{BufRead, Error, Write};
use std::collections::VecDeque;
use bufstream::BufStream;

#[derive(Debug, PartialEq)]
enum Command {
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
    messages: VecDeque<String>,
}

impl Redisish {
    pub fn new(messages: VecDeque<String>) -> Self {
        Self { messages: messages }
    }

    pub fn handle_client(&mut self, stream: TcpStream) -> Result<(), Error> {
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

    fn get(&mut self, stream: &mut BufStream<TcpStream>) -> Result<(), Error> {
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
