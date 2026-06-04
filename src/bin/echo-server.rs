use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        println!("hello there");

        tokio::spawn(async move {
            let mut buf = vec![0; 10];

            loop {
                println!("buf: {:?}", buf);

                match socket.read(&mut buf).await {
                    // Return value of `Ok(0)` signifies that the remote has
                    // closed
                    Ok(0) => return,
                    Ok(n) => {
                        println!("buf before write: {:?}", buf);

                        if socket.write(&buf[..n]).await.is_err() {
                            // Unexpected socket error. There isn't much we can
                            // do here so just stop processing.
                            return;
                        }

                        println!("buf after write: {:?}", buf);
                    }
                    Err(_) => {
                        // Unexpected socket error. There isn't much we can do
                        // here so just stop processing.
                        return;
                    }
                }
            }
        });
    }
}
