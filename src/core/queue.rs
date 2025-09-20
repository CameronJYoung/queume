pub enum QueueInputError {}

pub enum QueueOutputError {}

#[async_trait::async_trait]
pub trait Queue<T: Send + 'static> {
    fn input(&self, item: T) -> Result<(), QueueInputError>;

    fn output(&self) -> Result<T, QueueOutputError>;
}
