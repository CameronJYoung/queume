use std::collections::VecDeque;

use crate::core::{
    config::QueueConfig,
    message::Message,
    queue::{Queue, QueueInputError, QueueOutputError},
};

#[derive(Default)]
pub struct LifoQueue<T> {
    queue_config: QueueConfig,
    items: VecDeque<Message<T>>,
}

impl<T> LifoQueue<T> {
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
impl<T: Send> Queue<T> for LifoQueue<T> {
    async fn input(&mut self, item: Message<T>) -> Result<(), QueueInputError> {
        if self.queue_config.max_size.is_some_and(|m| self.len() == m) {
            return Err(QueueInputError::MaxSize);
        }

        self.items.push_front(item);

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

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_initial_len_zero() {
        let queue: LifoQueue<String> = LifoQueue::default();

        assert_eq!(queue.len(), 0);
    }

    #[tokio::test]
    async fn test_clear_queue() {
        let mut queue: LifoQueue<String> = LifoQueue::default();

        let message = Message::new("test_id".to_string(), 1, "test".to_string());

        queue.input(message).await.unwrap();

        queue.clear();

        assert_eq!(queue.len(), 0);
    }

    #[tokio::test]
    async fn test_input_adds_item_to_queue() {
        let mut queue: LifoQueue<String> = LifoQueue::default();

        let message = Message::new("test_id".to_string(), 1, "test".to_string());

        queue.input(message).await.unwrap();

        assert_eq!(queue.len(), 1);
    }

    #[tokio::test]
    async fn test_input_after_max_size_errors() {
        let queue_config = QueueConfig { max_size: Some(1) };
        let mut queue: LifoQueue<String> = LifoQueue::new(queue_config);

        let message_one = Message::new("test_id_one".to_string(), 1, "test".to_string());
        let message_two = Message::new("test_id_two".to_string(), 2, "test2".to_string());

        queue.input(message_one).await.unwrap();
        let out = queue.input(message_two).await;

        assert_eq!(queue.len(), 1);
        assert_eq!(out, Err(QueueInputError::MaxSize));
    }

    #[tokio::test]
    async fn test_output_returns_item() {
        let mut queue: LifoQueue<String> = LifoQueue::default();

        let message = Message::new("test_id".to_string(), 1, "test".to_string());
        let message_clone = message.clone();

        queue.input(message).await.unwrap();

        let out = queue.output().await.unwrap();

        assert_eq!(out, message_clone);
    }

    #[tokio::test]
    async fn test_output_returns_last_item() {
        let mut queue: LifoQueue<String> = LifoQueue::default();

        let message_one = Message::new("test_id_one".to_string(), 1, "test".to_string());
        let message_two = Message::new("test_id_two".to_string(), 2, "test2".to_string());
        let message_two_clone = message_two.clone();

        queue.input(message_one).await.unwrap();
        queue.input(message_two).await.unwrap();

        let out = queue.output().await.unwrap();

        assert_eq!(out, message_two_clone);
    }

    #[tokio::test]
    async fn test_output_removes_item() {
        let mut queue: LifoQueue<String> = LifoQueue::default();

        let message = Message::new("test_id".to_string(), 1, "test".to_string());

        queue.input(message).await.unwrap();

        let _ = queue.output().await.unwrap();

        assert_eq!(queue.len(), 0);
    }

    #[tokio::test]
    async fn test_output_returns_empty_err() {
        let mut queue: LifoQueue<String> = LifoQueue::default();

        let out = queue.output().await;

        assert_eq!(out, Err(QueueOutputError::Empty));
    }
}
