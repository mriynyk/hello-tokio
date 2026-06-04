use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = Vec::new();

    // let mut buffer = [0; 10];

    // read up to 10 bytes
    // let n = f.read(&mut buffer[..]).await?;

    // read the whole file
    f.read_to_end(&mut buffer).await?;

    println!("The bytes: {:?}", buffer);
    Ok(())
}