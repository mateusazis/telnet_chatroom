use crate::asynced::participant::Message;
use crate::asynced::participant::Participant;
use crate::asynced::participant::ParticipantInfo;
use async_std::io::prelude::BufReadExt;
use async_std::io::BufReader;
use async_std::io::BufWriter;
use async_std::net::TcpStream;
use async_std::stream::StreamExt;
use futures::channel::mpsc::UnboundedSender;
use futures::AsyncWriteExt;
use std::clone::Clone;
use std::collections::HashMap;

pub struct Server {
    sender: UnboundedSender<Message>,
    write_streams: HashMap<i32, BufWriter<TcpStream>>,
    participants: HashMap<i32, ParticipantInfo>,
}

impl Server {
    pub fn new(sender: UnboundedSender<Message>) -> Server {
        Server {
            sender,
            write_streams: HashMap::new(),
            participants: HashMap::new(),
        }
    }

    pub async fn handle_incoming_messages(&mut self, message: Message) -> std::io::Result<usize> {
        for (id, write_stream) in self.write_streams.iter_mut() {
            if id != &message.author.id {
                let content = &message.content;
                write_stream.write_all(content.as_bytes()).await?;
            }
        }
        self.participants.insert(message.author.id, message.author);
        Ok(0)
    }

    pub async fn remove(&mut self, id: &i32) {
        let mut stream = self.write_streams.remove(id).unwrap();
        stream.flush().await.unwrap();
        self.participants.remove(id).unwrap();
    }

    pub async fn handle_client(
        &mut self,
        stream: async_std::net::TcpStream,
    ) -> std::io::Result<Participant> {
        let mut write_stream = BufWriter::new(stream.clone());
        let buffer = BufReader::new(stream);
        let mut lines = buffer.lines();

        write_stream.write_all(b"What is your name?\n").await?;
        write_stream.flush().await?;
        let name = lines.next().await.unwrap().unwrap();

        let id = rand::random::<i32>();

        let mut initial_message = format!("Welcome to the chat room, {}!", name);
        if self.write_streams.is_empty() {
            initial_message += " There is no else here. You can send new messages anytime.";
        } else {
            initial_message.push_str(
                format!("There are {} other people here: ", self.write_streams.len()).as_str(),
            );
            for (_, participant) in self.participants.iter() {
                initial_message.push_str(
                    format!(
                        "\n\t{} ({})",
                        participant.name, participant.number_of_messages
                    )
                    .as_str(),
                );
            }
            initial_message.push('\n');
        }
        initial_message += "\n";
        write_stream.write_all(initial_message.as_bytes()).await?;

        let part = Participant::new(name, id, lines, self.sender.clone());
        self.write_streams.insert(id, write_stream);
        self.participants.insert(id, part.info.clone());
        Ok(part)
    }
}