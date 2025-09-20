pub enum QueueInputError {}

pub enum QueueOutputError {}

pub trait Queue {
    fn input(&self) -> Result<(), QueueInputError>;

    fn output(&self) -> Result<(), QueueOutputError>;
}
