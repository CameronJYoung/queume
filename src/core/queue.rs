pub enum QueueInputError {}

pub enum QueueOutputError {}

pub trait Queue<QueueItem> {
    fn input(&self) -> Result<(), QueueInputError>;

    fn output(&self) -> Result<QueueItem, QueueOutputError>;
}
