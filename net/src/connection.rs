use crate::message::{self, Message};
use bytes::{Buf, BufMut, BytesMut};
use log::info;
use tokio::io::{
    self, AsyncRead, AsyncReadExt, AsyncWrite, BufReader, BufWriter, ReadHalf, WriteHalf,
};

pub struct Connection<T> {
    reader: BufReader<ReadHalf<T>>,
    writer: BufWriter<WriteHalf<T>>,
    buffer: BytesMut,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;

impl<T: AsyncRead + AsyncWrite> Connection<T> {
    pub fn new(stream: T) -> Self {
        let (read, write) = io::split(stream);
        Self {
            reader: BufReader::new(read),
            writer: BufWriter::new(write),
            buffer: BytesMut::with_capacity(4 * 1024),
        }
    }

    pub async fn read_message(&mut self) -> Result<Option<String>, Error> {
        loop {
            if let Some(message) = self.parse_message()? {
                return Ok(Some(message));
            }
            if 0 == self.reader.read_buf(&mut self.buffer).await? {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("Connection reset by peer".into());
                }
            }
        }
    }

    fn parse_message(&mut self) -> Result<Option<String>, Error> {
        match Message::ready(&self.buffer) {
            Ok(_) => match Message::parse(&self.buffer) {
                Ok(Message::Document { content, byte_len }) => {
                    self.buffer.advance(byte_len);
                    Ok(Some(content))
                }
                Err(message::Error::Incomplete(m)) => {
                    info!("Parsing incomplete: {}", m);
                    Ok(None)
                }
                Err(message::Error::System(e)) => Err(e),
            },
            Err(_) => Ok(None),
        }
    }

    pub async fn write_message(&self, _message: &str) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::pin::Pin;
    use core::task::{Context, Poll};
    use tokio::io;
    // use tokio::stream::{Stream, StreamExt};

    struct MockStream<'a> {
        reader: Vec<&'a [u8]>,
        writer: Vec<u8>,
    }

    impl<'a> io::AsyncRead for MockStream<'a> {
        fn poll_read(
            mut self: Pin<&mut Self>,
            _cx: &mut Context,
            buf: &mut [u8],
        ) -> Poll<io::Result<usize>> {
            match self.reader.pop() {
                Some(content) => {
                    if content.len() > buf.len() {
                        for i in 0..buf.len() {
                            buf[i] = content[i];
                        }
                        Poll::Ready(Ok(buf.len()))
                    } else {
                        for i in 0..content.len() {
                            buf[i] = content[i];
                        }
                        Poll::Ready(Ok(content.len()))
                    }
                }
                None => Poll::Ready(Ok(0)),
            }
        }
    }

    impl<'a> io::AsyncWrite for MockStream<'a> {
        fn poll_write(
            mut self: Pin<&mut Self>,
            _cx: &mut Context,
            buf: &[u8],
        ) -> Poll<io::Result<usize>> {
            for item in buf {
                self.writer.push(*item);
            }
            Poll::Ready(Ok(buf.len()))
        }

        fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<io::Result<()>> {
            Poll::Ready(Ok(()))
        }

        fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<io::Result<()>> {
            Poll::Ready(Ok(()))
        }
    }

    fn create_connection(input: Vec<&[u8]>) -> Connection<MockStream> {
        let inner = MockStream {
            reader: input,
            writer: vec![],
        };
        Connection::new(inner)
    }

    #[tokio::test]
    async fn it_closes_down_with_nothing_to_read() {
        let mut conn = create_connection(vec![]);

        let res = conn.read_message().await;

        assert!(res.is_ok());
        assert!(res.unwrap().is_none());
    }

    #[tokio::test]
    async fn it_fails_if_buffer_is_partially_filled() {
        let mut conn = create_connection(vec![]);

        conn.buffer.put(&b"halfway done"[..]);

        let res = conn.read_message().await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn it_reads_a_message() {
        let inner = MockStream {
            reader: vec![b"type Object { name: String }\n"],
            writer: vec![],
        };
        let mut conn = Connection::new(inner);
        let res = conn.read_message().await;
        assert!(res.is_ok());
        assert!(res.unwrap().is_some());
    }

    #[test]
    fn it_attempts_to_parse_a_message() {
        let mut conn = create_connection(vec![]);

        let res = conn.parse_message();
        assert!(res.is_ok());
        assert!(res.unwrap().is_none());
    }

    #[test]
    fn it_parses_a_message_when_ready() {
        let mut conn = create_connection(vec![]);

        conn.buffer.put(&b"type Object { name: String }\n"[..]);
        let res = conn.parse_message();

        assert!(res.is_ok());
        let opt_message = res.unwrap();
        assert!(opt_message.is_some());
        assert_eq!(
            opt_message.unwrap(),
            String::from("type Object { name: String }"),
        )
    }

    #[tokio::test]
    async fn it_can_write_messages() {
        let inner = vec![];
        let mut conn = create_connection(inner);
        assert!(conn.write_message("OK").await.is_ok());
        // assert_eq!(inner, b"OK"[..]);
    }
}
