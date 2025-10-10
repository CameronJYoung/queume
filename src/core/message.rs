#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct Message<T> {
    pub id: String,
    pub unix_timestamp: i64,
    pub sequence_count: u64,
    pub payload: T,
}

impl<T> Message<T> {
    pub fn new(id: String, timestamp: i64, sequence_count: u64, payload: T) -> Self {
        Self {
            id,
            sequence_count,
            unix_timestamp: timestamp,
            payload,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_message() {
        let message = Message::new("test_id".to_string(), 1, 1, "payload");

        assert_eq!(message.id, "test_id".to_string());
        assert_eq!(message.unix_timestamp, 1);
        assert_eq!(message.payload, "payload")
    }
}
