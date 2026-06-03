use hello_tokio::Db;
use hello_tokio::process_socket::process;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    // A hashmap is used to store data
    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, socket_addr) = listener.accept().await.unwrap();
        println!("socket: {:?}", socket);
        println!("socket_addr: {:?}", socket_addr);

        // Clone the handle to the hash map.
        let db = db.clone();

        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move { process(socket, db).await });
    }
}
