use std::io::{BufReader, Lines};
use std::sync::mpsc::SyncSender;

pub struct Message {
    pub content: String,
    pub author: ParticipantInfo,
}

#[derive(Debug)]
pub struct ParticipantInfo {
    pub name: String,
    pub id: i32,
    pub number_of_messages: i32,
}

impl Clone for ParticipantInfo {
    fn clone(&self) -> Self {
        ParticipantInfo {
            name: self.name.clone(),
            id: self.id,
            number_of_messages: self.number_of_messages,
        }
    }
}

#[derive(Debug)]
pub struct Participant {
    pub info: ParticipantInfo,
    pub read_lines: Lines<BufReader<std::net::TcpStream>>,
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
            info: ParticipantInfo {
                name,
                id,
                number_of_messages: 0,
            },
            sender,
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
                self.info.name,
                self.info.number_of_messages + 1,
                line
            );
            self.info.number_of_messages += 1;
            self.sender
                .send(Message {
                    content: msg_out,
                    author: self.info.clone(),
                })
                .unwrap();
            line = self.read_line().expect("reading line");
        }

        Ok(0)
    }
}
