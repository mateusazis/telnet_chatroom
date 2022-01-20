use async_std::io::{BufReader, Lines};
use async_std::stream::StreamExt;
use futures::channel::mpsc::Sender;

#[derive(Debug)]
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
    pub read_lines: Lines<BufReader<async_std::net::TcpStream>>,
    pub sender: Sender<Message>,
}

impl Participant {
    pub fn new(
        name: String,
        id: i32,
        read_lines: Lines<BufReader<async_std::net::TcpStream>>,
        sender: Sender<Message>,
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

    pub async fn read_line(&mut self) -> Result<String, String> {
        let line = self.read_lines.next().await;
        if let Some(Ok(line2)) = line {
            return Ok(line2);
        }
        Err(String::from("something went wrong"))
    }

    pub async fn run_loop(&mut self) -> std::io::Result<usize> {
        let mut line = self.read_line().await.expect("reading line");
        while !line.eq("quit") {
            let msg_out = format!(
                "{} ({}): {}\n",
                self.info.name,
                self.info.number_of_messages + 1,
                line
            );
            self.info.number_of_messages += 1;
            self.sender
                .start_send(Message {
                    content: msg_out,
                    author: self.info.clone(),
                })
                .unwrap();
            line = self.read_line().await.expect("reading line");
        }

        Ok(0)
    }
}
