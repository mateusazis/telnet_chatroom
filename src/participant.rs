use std::io::{BufReader, Lines};
use std::sync::mpsc::SyncSender;

pub struct Message {
    pub content: String,
    pub author_id: i32,
}

#[derive(Debug)]
pub struct Participant {
    pub name: String,
    pub id: i32,
    pub read_lines: Lines<BufReader<std::net::TcpStream>>,
    pub number_of_messages: i32,
    pub sender: SyncSender<Message>,
}

impl Participant {
    pub fn new(
        name: String,
        id: i32,
        read_lines: Lines<BufReader<std::net::TcpStream>>,
        sender: SyncSender<Message>,
    ) -> Participant {
        Participant {
            name,
            id,
            sender,
            number_of_messages: 0,
            read_lines,
        }
    }

    pub fn read_line(&mut self) -> std::io::Result<String> {
        self.read_lines.next().unwrap()
    }

    pub fn run_loop(&mut self) -> std::io::Result<usize> {
        let mut line = self.read_line().expect("reading line");
        while !line.eq("quit") {
            let msg_out = format!(
                "{} ({}): {}\n",
                self.name,
                self.number_of_messages + 1,
                line
            );
            self.number_of_messages += 1;
            self.sender
                .send(Message {
                    content: msg_out,
                    author_id: self.id,
                })
                .unwrap();
            line = self.read_line().expect("reading line");
        }

        Ok(0)
    }
}
