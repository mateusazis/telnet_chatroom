use async_std::stream::StreamExt;
use async_std::sync::Mutex;
use futures::executor::block_on;
use std::sync::Arc;

async fn run_participant(
    server: Arc<Mutex<crate::asynced::server::Server>>,
    stream: async_std::net::TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    let participant = server.lock().await.handle_client(stream).await?;
    if let None = participant {
        return Ok(());
    }
    let mut participant = participant.unwrap();

    participant.run_loop().await?;

    server.lock().await.remove(&participant.info.id);
    Ok(())
}

fn spawn_participant(
    server: Arc<Mutex<crate::asynced::server::Server>>,
    stream: async_std::net::TcpStream,
) {
    let future = async move { run_participant(server, stream).await.expect("") };
    async_std::task::spawn(future);
}

async fn serv() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = futures::channel::mpsc::unbounded();
    let server = Arc::new(Mutex::new(crate::asynced::server::Server::new(tx)));
    let server_clone = server.clone();
    let listener = async_std::net::TcpListener::bind("127.0.0.1:8080").await?;

    async_std::task::spawn(async move {
        let mut incoming = listener.incoming();
        while let Some(stream) = incoming.next().await {
            let stream = stream.unwrap();
            let server = server.clone();
            spawn_participant(server, stream);
        }
    });

    loop {
        if let Some(event) = &rx.next().await {
            let mut s = server_clone.lock().await;
            s.handle_event(event).await?;
        }
    }
}

pub fn main() {
    block_on(serv()).expect("server should run");
}
