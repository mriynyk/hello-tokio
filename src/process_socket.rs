use mini_redis::Command::{self, Get, Set};
use mini_redis::{Connection, Frame};
use tokio::net::TcpStream;

use crate::Db;

pub async fn process(socket: TcpStream, db: Db) {
    // Connection, provided by `mini-redis`, handles parsing frames from
    // the socket
    let mut connection = Connection::new(socket);

    // Use `read_frame` to receive a command from the connection.
    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("Got: {:?}", frame);

        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                println!("set: {:?}", cmd);

                let mut db = db.lock().unwrap();

                db.insert(cmd.key().to_string(), cmd.value().clone());

                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                println!("set: {:?}", cmd);

                let db = db.lock().unwrap();

                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => {
                panic!("unimplemented {:?}", cmd)
            }
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}
