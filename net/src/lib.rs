mod connection;
pub mod handlers;
mod message;
pub mod tcp;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
