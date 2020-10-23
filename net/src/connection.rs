use crate::message::Message;
use bytes::{BufMut, BytesMut};
use tokio::io::{self, AsyncRead, AsyncWrite, BufReader, BufWriter, ReadHalf, WriteHalf};

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

    pub async fn read_message(&mut self) -> Result<Option<Message>, Error> {
        //     loop {
        //         if let Some(message) = self.parse_message()? {
        //             return Ok(Some(message));
        //         }
        //         if 0 == self.reader.read_buf(&mut self.buffer).await? {
        //             if self.buffer.is_empty() {
        //                 return Ok(None);
        //             } else {
        //                 return Err("Connection reset by peer".into());
        //             }
        //         }
        //     }
        if self.buffer.is_empty() {
            Ok(None)
        } else {
            Err("Connection reset by peer".into())
        }
    }

    // fn parse_message(&mut self) -> Result<Option<Message>, String> {
    //     unimplemented!()
    // }
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

    #[tokio::test]
    async fn it_closes_down_with_nothing_to_read() {
        let inner = MockStream {
            reader: vec![],
            writer: vec![],
        };
        let mut conn = Connection::new(inner);
        let res = conn.read_message().await;
        assert!(res.is_ok());
        assert!(res.unwrap().is_none());
    }

    #[tokio::test]
    async fn it_fails_if_buffer_is_partially_filled() {
        let inner = MockStream {
            reader: vec![],
            writer: vec![],
        };
        let mut conn = Connection::new(inner);
        conn.buffer.put(&b"halfway done"[..]);
        let res = conn.read_message().await;
        assert!(res.is_err());
    }

    // #[tokio::test]
    // async fn it_reads_a_message() {
    //     let inner = MockStream {
    //         reader: Vec::from(b"type Object { name: String }"),
    //         writer: vec![],
    //     };
    //     let mut conn = Connection::new(inner);
    // }
}
