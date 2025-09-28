use crate::core::message::Message;

#[derive(Debug, Eq, PartialEq)]
pub enum QueueInputError {
    MaxSize,
}

#[derive(Debug, Eq, PartialEq)]
pub enum QueueOutputError {
    Empty,
}

#[async_trait::async_trait]
pub trait Queue<T: Send> {
    async fn input(&mut self, item: Message<T>) -> Result<(), QueueInputError>;

    async fn output(&mut self) -> Result<Message<T>, QueueOutputError>;
}
