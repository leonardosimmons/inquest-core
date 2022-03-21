use bytes::Buf;

#[derive(Clone, std::fmt::Debug)]
pub enum ByteData {
    Simple(String),
    Error(String),
    Integer(u64),
    Bulk(bytes::Bytes),
    Null,
    Array(Vec<ByteData>),
}
impl ByteData {
    /// Creates an ByteData::Array frame
    fn array() -> ByteData {
        ByteData::Array(vec![])
    }

    /// Adds ByteData::Bulk type to ByteData::Array
    fn push_bulk(&mut self, data: bytes::Bytes) {
        match self {
            ByteData::Array(vec) => vec.push(ByteData::Bulk(data)),
            _ => panic!("not an array frame"),
        }
    }

    /// Adds ByteData::Integer type to ByteData::Array
    fn push_int(&mut self, data: u64) {
        match self {
            ByteData::Array(vec) => vec.push(ByteData::Integer(data)),
            _ => panic!("not and array frame"),
        }
    }

    /// Checks if entire message can be decoded form `src`
    pub fn check(src: &mut std::io::Cursor<&[u8]>) -> Result<(), ByteDataError> {
        match ByteData::read_byte(src)? {
            b'+' => {
                ByteData::read_bytes_line(src)?;
                Ok(())
            }
            b'-' => {
                ByteData::read_bytes_line(src)?;
                Ok(())
            }
            b'$' => {
                if b'-' == ByteData::peek_byte(src)? {
                    // skip '-1\r\n'
                    ByteData::skip_buffer(src, 4)
                } else {
                    let len: usize = ByteData::read_newline_decimal(src)?.try_into()?;

                    // skip that number of bytes +2 (\r\n)
                    ByteData::skip_buffer(src, len + 2)
                }
            }
            b'*' => {
                let len = ByteData::read_newline_decimal(src)?;

                for _ in 0..len {
                    ByteData::check(src)?;
                }
                Ok(())
            }
            actual => Err(format!("protocol error; invalid frame type btye `{}`", actual).into()),
        }
    }

    /// The message has already been validated by check
    pub fn parse(src: &mut std::io::Cursor<&[u8]>) -> Result<ByteData, ByteDataError> {
        match ByteData::read_byte(src)? {
            b'+' => {
                // Read the line and convert to `Vec<u8>`
                let line = ByteData::read_bytes_line(src)?.to_vec();

                // Convert line to string
                let string = String::from_utf8(line)?;
                Ok(ByteData::Simple(string))
            }
            b'-' => {
                let line = ByteData::read_bytes_line(src)?.to_vec();
                let string = String::from_utf8(line)?;
                Ok(ByteData::Error(string))
            }
            b':' => {
                let len = ByteData::read_newline_decimal(src)?;
                Ok(ByteData::Integer(len))
            }
            b'$' => {
                if b'-' == ByteData::peek_byte(src)? {
                    let line = ByteData::read_bytes_line(src)?;

                    if line != b"-1" {
                        return Err("protocol error; invalid frame format".into());
                    }
                    Ok(ByteData::Null)
                } else {
                    // Reads the bulk string
                    let len = ByteData::read_newline_decimal(src)?.try_into()?;
                    let n = len + 2;

                    if src.remaining() < n {
                        return Err(ByteDataError::Incomplete);
                    }

                    let data = bytes::Bytes::copy_from_slice(&src.chunk()[..len]);

                    // skip that number of bytes +2 ('\r\n)
                    ByteData::skip_buffer(src, n)?;
                    Ok(ByteData::Bulk(data))
                }
            }
            b'*' => {
                let len = ByteData::read_newline_decimal(src)?.try_into()?;
                let mut out = Vec::with_capacity(len);

                for _ in 0..len {
                    out.push(ByteData::parse(src)?);
                }
                Ok(ByteData::Array(out))
            }
            _ => unimplemented!(),
        }
    }
}

impl PartialEq<&str> for ByteData {
    fn eq(&self, other: &&str) -> bool {
        match self {
            ByteData::Simple(s) => s.eq(other),
            ByteData::Bulk(s) => s.eq(other),
            _ => false,
        }
    }
}

impl std::fmt::Display for ByteData {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::str;

        match self {
            ByteData::Simple(response) => response.fmt(fmt),
            ByteData::Error(msg) => write!(fmt, "error: {}", msg),
            ByteData::Integer(num) => num.fmt(fmt),
            ByteData::Bulk(msg) => match str::from_utf8(msg) {
                Ok(string) => string.fmt(fmt),
                Err(_) => write!(fmt, "{:?}", msg),
            },
            ByteData::Null => "(nil)".fmt(fmt),
            ByteData::Array(parts) => {
                for (i, part) in parts.iter().enumerate() {
                    if i > 0 {
                        write!(fmt, " ")?;
                        part.fmt(fmt)?;
                    }
                }

                Ok(())
            }
        }
    }
}

impl ByteController for ByteData {
    fn peek_byte(src: &mut std::io::Cursor<&[u8]>) -> Result<u8, ByteDataError> {
        if !src.has_remaining() {
            return Err(ByteDataError::Incomplete);
        }
        Ok(src.chunk()[0])
    }

    fn read_byte(src: &mut std::io::Cursor<&[u8]>) -> Result<u8, ByteDataError> {
        if !src.has_remaining() {
            return Err(ByteDataError::Incomplete);
        }
        Ok(src.get_u8())
    }

    fn skip_buffer(src: &mut std::io::Cursor<&[u8]>, n: usize) -> Result<(), ByteDataError> {
        if src.remaining() < n {
            return Err(ByteDataError::Incomplete);
        }
        Ok(src.advance(n))
    }

    fn read_newline_decimal(src: &mut std::io::Cursor<&[u8]>) -> Result<u64, ByteDataError> {
        use atoi::atoi;
        let line = ByteData::read_bytes_line(src)?;
        atoi::<u64>(line).ok_or_else(|| "protocol error; invalid frame format".into())
    }

    fn read_bytes_line<'a>(src: &mut std::io::Cursor<&'a [u8]>) -> Result<&'a [u8], ByteDataError> {
        let start = src.position() as usize;
        let end = src.get_ref().len() - 1;

        for i in start..end {
            if src.get_ref()[i] == b'\r' && src.get_ref()[i + 1] == b'\n' {
                src.set_position((i + 2) as u64);

                return Ok(&src.get_ref()[start..i]);
            }
        }

        Err(ByteDataError::Incomplete)
    }
}

#[derive(Debug)]
pub enum ByteDataError {
    Incomplete,
    Other(String),
}

impl From<String> for ByteDataError {
    fn from(src: String) -> ByteDataError {
        ByteDataError::Other(src.into())
    }
}

impl From<&str> for ByteDataError {
    fn from(src: &str) -> ByteDataError {
        src.to_string().into()
    }
}

impl From<std::string::FromUtf8Error> for ByteDataError {
    fn from(_src: std::string::FromUtf8Error) -> ByteDataError {
        "protocol error; invalid buffer format".into()
    }
}

impl From<std::num::TryFromIntError> for ByteDataError {
    fn from(_src: std::num::TryFromIntError) -> ByteDataError {
        "protocol error; invalid buffer format".into()
    }
}

impl std::error::Error for ByteDataError {}

impl std::fmt::Display for ByteDataError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ByteDataError::Incomplete => "stream ended early".fmt(fmt),
            ByteDataError::Other(err) => err.fmt(fmt),
        }
    }
}

pub(crate) trait ByteController {
    /// Returns first byte in the buffer
    fn peek_byte(src: &mut std::io::Cursor<&[u8]>) -> Result<u8, ByteDataError>;

    /// Returns single byte at cursor location
    fn read_byte(src: &mut std::io::Cursor<&[u8]>) -> Result<u8, ByteDataError>;

    /// Skips buffer cursor to specified location
    fn skip_buffer(src: &mut std::io::Cursor<&[u8]>, n: usize) -> Result<(), ByteDataError>;

    /// Reads a new-line terminated decimal
    fn read_newline_decimal(src: &mut std::io::Cursor<&[u8]>) -> Result<u64, ByteDataError>;

    /// Returns single line of bytes starting at cursor location
    fn read_bytes_line<'a>(src: &mut std::io::Cursor<&'a [u8]>) -> Result<&'a [u8], ByteDataError>;
}
