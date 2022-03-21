#![allow(unused)]
use bytes;
use bytes::Buf;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::byte_data::{ByteData, ByteDataError};

const DEFAULT_CONNECTION_BUFFER_SIZE: usize = 8 * 1024;

#[derive(Debug)]
struct Connection {
    stream: tokio::io::BufWriter<tokio::net::TcpStream>, // write
    buffer: bytes::BytesMut // read
}
impl Connection {
    pub fn new(socket: tokio::net::TcpStream) -> Connection {
        Connection {
            stream: tokio::io::BufWriter::new(socket),
            buffer: bytes::BytesMut::with_capacity(DEFAULT_CONNECTION_BUFFER_SIZE)
        }
    }

    pub async fn read_frame(&mut self) -> Result<Option<ByteData>, ByteDataError> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame))
            }

            // Try and read more data if parse_frame() fails -> `0` == end of stream
            if 0 == self.stream.read_buf(&mut self.buffer).await.unwrap() {
                return if self.buffer.is_empty() {
                    Ok(None)
                } else {
                    Err("connection reset".into())
                }
            }
        }
    }

    fn parse_frame(&mut self) -> Result<Option<ByteData>, ByteDataError> {
        use crate::byte_data::ByteDataError::Incomplete;
        // Buffer is wrapped with cursor in order to track the current location of the buffer
        let mut buf = std::io::Cursor::new(&self.buffer[..]);

        // Checks to see if enough data has been buffered to parse a single frame
        match ByteData::check(&mut buf) {
            Ok(_) => {
                let len = buf.position() as usize;

                // reset buffer position back to start
                buf.set_position(0);

                let frame = ByteData::parse(&mut buf)?;

                // discard parsed data from buffer (cursor is moved in ByteData::parse())
                self.buffer.advance(len);
                Ok(Some(frame))
            }
            Err(Incomplete) => Ok(None),
            Err(err) => Err(err.into())
        }
    }

    pub async fn write_frame(&mut self, frame: &ByteData) -> std::io::Result<()> {
        match frame {
            ByteData::Array(data) => {
                // Encode the frame type prefix -> array == `*`
                self.stream.write_u8(b'*').await?;

                // encode the length of the array
                self.write_decimal(data.len() as u64).await?;

                // Iterate and encode each entry within the array
                for entry in &**data {
                    self.write_value(entry).await?;
                }
            }
            // if frame type is a literal, encode value directly
            _ => self.write_value(frame).await?
        }

        // write the remaining contents to the buffer -> ensures the encoded frame is written to socket
        self.stream.flush().await
    }

    async fn write_value(&mut self, frame: &ByteData) -> std::io::Result<()> {
        match frame {
            ByteData::Simple(data) => {
                self.stream.write_u8(b'+').await?;
                self.stream.write_all(data.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            ByteData::Error(data) => {
                self.stream.write_u8(b'-').await?;
                self.stream.write_all(data.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            ByteData::Integer(data) => {
                self.stream.write_u8(b':').await?;
                self.write_decimal(*data).await?;
            }
            ByteData::Null => {
                self.stream.write_all(b"$-1\r\n").await?;
            }
            ByteData::Bulk(data) => {
                let len = data.len();

                self.stream.write_u8(b'$').await?;
                self.write_decimal(len as u64).await?;
                self.stream.write_all(data).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            ByteData::Array(_data) => unreachable!()
        }
        Ok(())
    }

    /// Write a decimal frame to the stream
    async fn write_decimal(&mut self, data: u64) -> std::io::Result<()> {
        use std::io::Write;

        // convert value to string
        let mut buf = [0u8; 20];
        let mut buf = std::io::Cursor::new(&mut buf[..]);
        write!(&mut buf, "{}", data);

        // get current buffer location and retrieve data up until that point
        let pos = buf.position() as usize;
        self.stream.write_all(&buf.get_ref()[..pos]).await?;
        self.stream.write_all(b"\r\n").await?;

        Ok(())
    }
}
