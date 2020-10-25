use crate::connection::Error;
use bytes::BytesMut;
use std::io::Cursor;

pub struct Message {}

impl Message {
    pub fn ready(cursor: BytesMut) -> Result<(), Error> {
        let _unmatched_braces = cursor.iter().fold(0, |count, b| {
            if *b == b'{' {
                count + 1
            } else if *b == b'}' {
                count - 1
            } else {
                count
            }
        });
        Ok(())
    }
}
