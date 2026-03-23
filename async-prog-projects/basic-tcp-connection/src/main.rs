
use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;  // write entire buffer


#[tokio::main]
async fn main() {

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");

    loop{
        // multiple clients can connect to this server
        let (mut socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move{
            let response = "HTTP/1.1 200 OK\r\n\r\nHello, Rust Async!";

            // tcp sockets only understand bytes, so we need to convert string to bytes
            socket.write_all(response.as_bytes()).await.unwrap();
        });
    }
}