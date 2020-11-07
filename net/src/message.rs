use bytes::BytesMut;

#[derive(Debug, PartialEq)]
pub enum Message {
    Document { content: String, byte_len: usize },
}

#[derive(Debug)]
pub enum Error {
    Incomplete(String),
    System(crate::connection::Error),
}

impl Message {
    pub fn ready(cursor: &BytesMut) -> Result<(), Error> {
        if cursor.iter().find(|&&b| b == b'{').is_some() {
            Message::check_balanced_braces(cursor)
        } else if cursor.iter().find(|&&b| b == b'\n').is_some() {
            Ok(())
        } else {
            Err(Error::Incomplete(String::from(
                "Message currently not ready",
            )))
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
            Err(Error::Incomplete(String::from(
                "Unmatched braces. Message currently not ready",
            )))
        } else {
            Ok(())
        }
    }

    pub fn parse(cursor: &BytesMut) -> Result<Message, Error> {
        let mut last_closed: usize = 0;
        let mut first_closed: usize = 0;
        cursor.iter().fold((0, 0), |(index, unmatched), b| {
            if *b == b'{' {
                (index + 1, unmatched + 1)
            } else if *b == b'}' {
                let new_unmatched = unmatched - 1;
                if new_unmatched == 0 {
                    last_closed = index + 1;
                    if first_closed == 0 {
                        first_closed = last_closed;
                    }
                }
                (index + 1, new_unmatched)
            } else {
                (index + 1, unmatched)
            }
        });
        let slice = match cursor[0] {
            b'{' => &cursor[..first_closed],
            _ => &cursor[..last_closed],
        };
        println!("Last index of closed brace: {}", last_closed);
        println!("Slice: {:?}", slice);
        match std::str::from_utf8(slice) {
            Ok(content) => Ok(Message::Document {
                content: String::from(content),
                byte_len: slice.len(),
            }),
            Err(e) => Err(Error::System(e.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::BytesMut;

    #[test]
    fn it_checks_for_an_open_brace() {
        let buf = BytesMut::from("{}");
        assert!(Message::ready(&buf).is_ok());

        let buf = BytesMut::from("type Object");
        assert!(Message::ready(&buf).is_err());
    }

    #[test]
    fn it_checks_for_a_new_line_if_no_brace() {
        let buf = BytesMut::from("scalar Date\n");
        assert!(Message::ready(&buf).is_ok());

        let buf = BytesMut::from("union Pet = Dog | Cat |");
        assert!(Message::ready(&buf).is_err());
    }

    #[test]
    fn it_checks_that_all_braces_are_paired() {
        let buf = BytesMut::from("{ user { }");
        assert!(Message::ready(&buf).is_err());
    }

    #[test]
    fn it_checks_that_only_first_brace_must_be_paired() {
        let buf = BytesMut::from("type User { name: String, email: Address }\ntype Address {\n");
        assert!(Message::ready(&buf).is_ok());
    }

    #[test]
    fn it_parses_a_message() {
        let buf = BytesMut::from("type User {\n name: String,\n email: Email,\n}");
        let parsed = Message::parse(&buf);
        assert!(parsed.is_ok());
        assert_eq!(
            parsed.unwrap(),
            Message::Document {
                content: String::from_utf8(buf.to_vec()).unwrap(),
                byte_len: buf.len(),
            }
        );
    }

    #[test]
    fn it_only_parses_complete_messages() {
        let buf = BytesMut::from(
            r#"
type User {
    name: String
    email: Email
}

type Admin {
    user: User
    priveledges: [Priviledges]!
}

type Incomplete {
"#,
        );
        let parsed = Message::parse(&buf);
        assert!(parsed.is_ok());
        assert_eq!(
            parsed.unwrap(),
            Message::Document {
                content: String::from(
                    r#"
type User {
    name: String
    email: Email
}

type Admin {
    user: User
    priveledges: [Priviledges]!
}"#
                ),
                byte_len: 111
            }
        );
    }

    #[test]
    fn it_only_parses_a_query() {
        let buf = BytesMut::from(
            r#"{ user { name, email, permissions(role: "admin") { home, isSudo, } } }

type Login {
    user: User,
    expiry: DateTime,
}
"#,
        );
        let parsed = Message::parse(&buf);
        assert!(parsed.is_ok());
        assert_eq!(
            parsed.unwrap(),
            Message::Document {
                content: String::from(
                    "{ user { name, email, permissions(role: \"admin\") { home, isSudo, } } }"
                ),
                byte_len: 70,
            }
        );
    }
}
