use crate::participant::Message;
use crate::participant::Participant;
use std::clone::Clone;
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc::SyncSender;

pub struct Server {
    sender: SyncSender<Message>,
    write_streams: HashMap<i32, TcpStream>,
}

impl Server {
    pub fn new(sender: SyncSender<Message>) -> Server {
        Server {
            sender,
            write_streams: HashMap::new(),
        }
    }

    pub fn handle_incoming_messages(&mut self, message: Message) -> std::io::Result<usize> {
        for (id, write_stream) in self.write_streams.iter_mut() {
            if id != &message.author_id {
                let content = &message.content;
                write_stream.write(content.as_bytes())?;
            }
        }
        Ok(0)
    }

    pub fn remove(&mut self, id: &i32) {
        self.write_streams.remove(id).unwrap();
    }

    pub fn handle_client(
        &mut self,
        mut stream: std::net::TcpStream,
    ) -> std::io::Result<Participant> {
        let name = {
            stream.write(b"What is your name?\n")?;
            crate::io_utils::read_line(&mut stream).expect("reading line")
        };

        let id = rand::random::<i32>();
        let write_stream = stream.try_clone().unwrap();

        let part = Participant {
            name,
            id,
            read_stream: stream,
            number_of_messages: 0,
            sender: self.sender.clone(),
        };
        self.write_streams.insert(id, write_stream);
        Ok(part)
    }
}
