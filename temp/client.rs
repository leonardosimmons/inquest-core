#![allow(unused)]
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    // opens file "foo.txt"
    let mut f = File::open("foo.txt").await?;
    // creates a buffer containing a limit of 10 bytes
    //let mut buffer = [0; 10];
    let mut buffer = Vec::new();

    // read up to 10 bytes
    //let n = f.read(&mut buffer[..]).await?;
    f.read_to_end(&mut buffer).await?;

    println!("The bytes: {:?}", buffer);
    Ok(())
}

async fn write() {
    let mut f = File::create("foo2.txt").await?;

    // writes some prefix of the byte string, but not neccessarly all of it
    let n = f.write(b"some bytes to be written").await?;
}