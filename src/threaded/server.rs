use crate::threaded::participant::Message;
use crate::threaded::participant::Participant;
use crate::threaded::participant::ParticipantInfo;
use std::clone::Clone;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc::SyncSender;

pub struct Server {
    sender: SyncSender<Message>,
    write_streams: HashMap<i32, BufWriter<TcpStream>>,
    participants: HashMap<i32, ParticipantInfo>,
}

impl Server {
    pub fn new(sender: SyncSender<Message>) -> Server {
        Server {
            sender,
            write_streams: HashMap::new(),
            participants: HashMap::new(),
        }
    }

    pub fn handle_incoming_messages(&mut self, message: Message) -> std::io::Result<usize> {
        for (id, write_stream) in self.write_streams.iter_mut() {
            if id != &message.author.id {
                let content = &message.content;
                write_stream.write(content.as_bytes())?;
            }
        }
        self.participants.insert(message.author.id, message.author);
        Ok(0)
    }

    pub fn remove(&mut self, id: &i32) {
        let mut write_stream = self.write_streams.remove(id).unwrap();
        write_stream.flush().unwrap();
        self.participants.remove(id).unwrap();
    }

    pub fn handle_client(&mut self, stream: std::net::TcpStream) -> std::io::Result<Participant> {
        let mut write_stream = BufWriter::new(stream.try_clone().unwrap());
        let buffer = BufReader::new(stream);
        let mut lines = buffer.lines();

        write_stream.write_all(b"What is your name?\n")?;
        write_stream.flush()?;
        let name = lines.next().unwrap().unwrap();

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
        write_stream.write_all(initial_message.as_bytes())?;

        let part = Participant::new(name, id, lines, self.sender.clone());
        self.write_streams.insert(id, write_stream);
        self.participants.insert(id, part.info.clone());
        Ok(part)
    }
}
