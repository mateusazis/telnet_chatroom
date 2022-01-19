use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

fn serv() -> std::io::Result<usize> {
    let (tx, rx) = mpsc::sync_channel(1024 * 1024);
    let server = Arc::new(Mutex::new(crate::threaded::server::Server::new(tx)));
    let server_clone = server.clone();
    let listener = std::net::TcpListener::bind("127.0.0.1:8080").unwrap();
    listener.set_nonblocking(false).expect("set non blocking");

    std::thread::spawn(move || {
        let mut threads = Vec::new();
        for stream in listener.incoming() {
            let server = server.clone();
            let t = std::thread::spawn(move || {
                let mut participant = server
                    .lock()
                    .unwrap()
                    .handle_client(stream.unwrap())
                    .expect("handle");

                participant.run_loop().expect("run loop");

                server.lock().unwrap().remove(&participant.info.id);
            });
            threads.push(t);
        }

        for t in threads {
            t.join().unwrap();
        }
    });

    for message in rx {
        server_clone
            .lock()
            .unwrap()
            .handle_incoming_messages(message)?;
    }

    Ok(0)
}

pub fn main() {
    serv().expect("serv");
}
