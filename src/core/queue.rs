use crate::core::message::Message;

#[derive(Debug, Eq, PartialEq)]
pub enum QueueInputError {
    MaxSize,
}

#[derive(Debug, Eq, PartialEq)]
pub enum QueueOutputError {
    Empty,
}

#[derive(Debug, Eq, PartialEq)]
pub enum QueueSequenceCalcError {
    MaxSize,
}

#[async_trait::async_trait]
pub trait Queue<T: Send> {
    async fn input(&mut self, item: Message<T>) -> Result<(), QueueInputError>;

    async fn output(&mut self) -> Result<Message<T>, QueueOutputError>;
}

#[async_trait::async_trait]
pub trait OrderedQueue<T: Send> {
    async fn get_next_sequence_count(&mut self) -> Result<u64, QueueSequenceCalcError>;
}
