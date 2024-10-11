use tokio::{io::AsyncWriteExt, net::TcpListener};

use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("Server running on 127.0.0.1:8080");

    loop {
        match listener.accept().await {
            Ok((mut socket, addr)) => {
                println!("New client: {:?}", addr);
                tokio::spawn(async move {
                    let response = b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 25\r\n\r\n{\"data\": \"Hello, World!\"}";

                    if let Err(e) = socket.write_all(response).await {
                        println!("Failed to write to socket: {:?}", e);
                    }

                    println!("Connection closed");
                });
            }
            Err(e) => println!("Couldn't get client: {:?}", e),
        }
    }
}
