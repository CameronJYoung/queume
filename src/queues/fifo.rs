use std::collections::VecDeque;

use crate::core::{
    config::QueueConfig,
    message::Message,
    queue::{OrderedQueue, Queue, QueueInputError, QueueOutputError, QueueSequenceCalcError},
};

#[derive(Default)]
pub struct FifoQueue<T> {
    queue_config: QueueConfig,
    items: VecDeque<Message<T>>,
}

impl<T> FifoQueue<T> {
    pub fn new(queue_config: QueueConfig) -> Self {
        Self {
            queue_config,
            items: VecDeque::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

#[async_trait::async_trait]
impl<T: Send> Queue<T> for FifoQueue<T> {
    async fn input(&mut self, item: Message<T>) -> Result<(), QueueInputError> {
        if self.queue_config.max_size.is_some_and(|m| self.len() == m) {
            return Err(QueueInputError::MaxSize);
        }

        self.items.push_back(item);

        Ok(())
    }

    async fn output(&mut self) -> Result<Message<T>, QueueOutputError> {
        let out_item = self.items.pop_front();

        match out_item {
            Some(i) => Ok(i),
            None => Err(QueueOutputError::Empty),
        }
    }
}

#[async_trait::async_trait]
impl<T: Send> OrderedQueue<T> for FifoQueue<T> {
    async fn get_next_sequence_count(&mut self) -> Result<u64, QueueSequenceCalcError> {
        if self.queue_config.max_size.is_some_and(|m| self.len() == m) {
            return Err(QueueSequenceCalcError::MaxSize);
        }

        let final_item = self.items.front();

        match final_item {
            Some(i) => Ok(i.sequence_count + 1),
            None => Ok(1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_initial_len_zero() {
        let queue: FifoQueue<String> = FifoQueue::default();

        assert_eq!(queue.len(), 0);
    }

    #[tokio::test]
    async fn test_clear_queue() {
        let mut queue: FifoQueue<String> = FifoQueue::default();

        let message = Message::new("test_id".to_string(), 1, 1, "test".to_string());

        queue.input(message).await.unwrap();

        queue.clear();

        assert_eq!(queue.len(), 0);
    }

    #[tokio::test]
    async fn test_input_adds_item_to_queue() {
        let mut queue: FifoQueue<String> = FifoQueue::default();

        let message = Message::new("test_id".to_string(), 1, 1, "test".to_string());

        queue.input(message).await.unwrap();

        assert_eq!(queue.len(), 1);
    }

    #[tokio::test]
    async fn test_input_after_max_size_errors() {
        let queue_config = QueueConfig { max_size: Some(1) };
        let mut queue: FifoQueue<String> = FifoQueue::new(queue_config);

        let message_one = Message::new("test_id_one".to_string(), 1, 1, "test".to_string());
        let message_two = Message::new("test_id_two".to_string(), 2, 1, "test2".to_string());

        queue.input(message_one).await.unwrap();
        let out = queue.input(message_two).await;

        assert_eq!(queue.len(), 1);
        assert_eq!(out, Err(QueueInputError::MaxSize));
    }

    #[tokio::test]
    async fn test_output_returns_item() {
        let mut queue: FifoQueue<String> = FifoQueue::default();

        let message = Message::new("test_id".to_string(), 1, 1, "test".to_string());
        let message_clone = message.clone();

        queue.input(message).await.unwrap();

        let out = queue.output().await.unwrap();

        assert_eq!(out, message_clone);
    }

    #[tokio::test]
    async fn test_output_returns_first_item() {
        let mut queue: FifoQueue<String> = FifoQueue::default();

        let message_one = Message::new("test_id_one".to_string(), 1, 1, "test".to_string());
        let message_two = Message::new("test_id_two".to_string(), 2, 2, "test2".to_string());

        let message_one_clone = message_one.clone();

        queue.input(message_one).await.unwrap();
        queue.input(message_two).await.unwrap();

        let out = queue.output().await.unwrap();

        assert_eq!(out, message_one_clone);
    }

    #[tokio::test]
    async fn test_output_removes_item() {
        let mut queue: FifoQueue<String> = FifoQueue::default();

        let message = Message::new("test_id".to_string(), 1, 2, "test".to_string());

        queue.input(message).await.unwrap();

        let _ = queue.output().await.unwrap();

        assert_eq!(queue.len(), 0);
    }

    #[tokio::test]
    async fn test_output_returns_empty_err() {
        let mut queue: FifoQueue<String> = FifoQueue::default();

        let out = queue.output().await;

        assert_eq!(out, Err(QueueOutputError::Empty));
    }

    #[tokio::test]
    async fn test_get_next_sequence_count_max_size() {
        let queue_config = QueueConfig { max_size: Some(1) };
        let mut queue: FifoQueue<String> = FifoQueue::new(queue_config);

        let message = Message::new("test_id".to_string(), 1, 2, "test".to_string());

        queue.input(message).await.unwrap();

        let next_seq = queue.get_next_sequence_count().await;

        assert_eq!(next_seq, Err(QueueSequenceCalcError::MaxSize));
    }

    #[tokio::test]
    async fn test_get_next_sequence_count() {
        let mut queue: FifoQueue<String> = FifoQueue::default();

        let message = Message::new("test_id".to_string(), 1, 1, "test".to_string());

        queue.input(message).await.unwrap();

        let next_seq = queue.get_next_sequence_count().await;

        assert_eq!(next_seq, Ok(2));
    }
}
