#[derive(Debug, PartialEq)]
pub enum QueueInputError {}

#[derive(Debug, PartialEq)]
pub enum QueueOutputError {
    Empty,
}

#[async_trait::async_trait]
pub trait Queue<T: Send> {
    async fn input(&mut self, item: T) -> Result<(), QueueInputError>;

    async fn output(&mut self) -> Result<T, QueueOutputError>;
}
