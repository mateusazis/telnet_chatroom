use async_std::stream::StreamExt;
use async_std::sync::Mutex;
use futures::executor::block_on;
use std::sync::Arc;

async fn serv() -> std::io::Result<usize> {
    let (tx, mut rx) = futures::channel::mpsc::unbounded();
    let server = Arc::new(Mutex::new(crate::asynced::server::Server::new(tx)));
    let server_clone = server.clone();
    let listener = async_std::net::TcpListener::bind("127.0.0.1:8080").await?;

    async_std::task::spawn(async move {
        let mut incoming = listener.incoming();
        while let Some(stream) = incoming.next().await {
            let stream = stream.unwrap();

            let server = server.clone();
            async_std::task::spawn(async move {
                let mut participant = server.lock().await.handle_client(stream).await.unwrap();

                participant.run_loop().await.expect("run loop");

                server.lock().await.remove(&participant.info.id)
            });
        }
    });

    loop {
        if let Some(message) = rx.next().await {
            let mut s = server_clone.lock().await;
            s.handle_incoming_messages(message).await.unwrap();
        }
    }
}

pub fn main() {
    block_on(serv()).expect("server");
}
