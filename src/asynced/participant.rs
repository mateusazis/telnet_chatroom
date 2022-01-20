use async_std::io::{BufReader, Lines};
use async_std::stream::StreamExt;
use futures::channel::mpsc::UnboundedSender;

pub enum ExitType {
    ConnectionAborted,
    GracefulTermination,
}

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
    pub sender: UnboundedSender<Message>,
}

impl Participant {
    pub fn new(
        name: String,
        id: i32,
        read_lines: Lines<BufReader<async_std::net::TcpStream>>,
        sender: UnboundedSender<Message>,
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

    pub async fn read_line(&mut self) -> Result<Option<String>, String> {
        let line = self.read_lines.next().await;
        match line {
            Some(Ok(line2)) => return Ok(Some(line2)),
            None => Ok(None),
            Some(Err(err)) => {
                let err = err as async_std::io::Error;
                match err.kind() {
                    async_std::io::ErrorKind::ConnectionReset => Ok(None),
                    _default => Err(err.to_string()),
                }
            }
        }
    }

    pub async fn run_loop(&mut self) -> std::io::Result<ExitType> {
        let mut line_read = self.read_line().await.expect("reading line");
        while let Some(line) = line_read {
            if line.eq("quit") {
                return Ok(ExitType::GracefulTermination);
            }
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
                .expect("should notify of new message");
            line_read = self.read_line().await.expect("reading line");
        }

        Ok(ExitType::ConnectionAborted)
    }
}
