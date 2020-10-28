use crate::connection::Error;
use bytes::BytesMut;
use std::io::Cursor;

pub struct Message {}

impl Message {
    pub fn ready(cursor: &BytesMut) -> Result<(), Error> {
        if cursor.iter().find(|&&b| b == b'{').is_some() {
            Message::check_balanced_braces(cursor)
        } else if cursor.iter().find(|&&b| b == b'\n').is_some() {
            Ok(())
        } else {
            Err("Message currently not ready".into())
        }
    }

    fn check_balanced_braces(cursor: &BytesMut) -> Result<(), Error> {
        let mut stop_flag = false;
        let unmatched_braces = cursor.iter().fold(0, |count, b| {
            if stop_flag {
                count
            } else if *b == b'{' {
                count + 1
            } else if *b == b'}' {
                let new_count = count - 1;
                if new_count == 0 {
                    stop_flag = true;
                }
                count - 1
            } else {
                count
            }
        });
        if unmatched_braces > 0 {
            Err("Unmatched braces. Message currently not ready".into())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::{BufMut, BytesMut};

    #[test]
    fn it_checks_for_an_open_brace() {
        let mut buf = BytesMut::with_capacity(64);
        buf.put(&b"{}"[..]);
        assert!(Message::ready(&buf).is_ok());

        let mut buf = BytesMut::with_capacity(64);
        buf.put(&b"type Object"[..]);
        assert!(Message::ready(&buf).is_err());
    }

    #[test]
    fn it_checks_for_a_new_line_if_no_brace() {
        let mut buf = BytesMut::with_capacity(64);
        buf.put(&b"scalar Date\n"[..]);
        assert!(Message::ready(&buf).is_ok());

        let mut buf = BytesMut::with_capacity(64);
        buf.put(&b"union Pet = Dog | Cat |"[..]);
        assert!(Message::ready(&buf).is_err());
    }

    #[test]
    fn it_checks_that_all_braces_are_paired() {
        let mut buf = BytesMut::with_capacity(64);
        buf.put(&b"{ user { }"[..]);
        assert!(Message::ready(&buf).is_err());
    }

    #[test]
    fn it_checks_that_only_first_brace_must_be_paired() {
        let mut buf = BytesMut::with_capacity(64);
        buf.put(&b"type User { name: String, email: Address }\ntype Address {\n"[..]);
        assert!(Message::ready(&buf).is_ok());
    }
}
