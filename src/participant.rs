use std::string::FromUtf8Error;
use std::sync::mpsc::SyncSender;

pub struct Message {
    pub content: String,
    pub author_id: i32,
}

#[derive(Debug)]
pub struct Participant {
    pub name: String,
    pub id: i32,
    pub read_stream: std::net::TcpStream,
    pub write_stream: std::net::TcpStream,
    pub number_of_messages: i32,
    pub sender: SyncSender<Message>,
}

impl Participant {
    pub fn read_line(&mut self) -> Result<String, FromUtf8Error> {
        crate::io_utils::read_line(&mut self.read_stream)
    }

    pub fn run_loop(&mut self) -> std::io::Result<usize> {
        let mut line = self.read_line().expect("reading line");
        while !line.starts_with("quit") {
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
