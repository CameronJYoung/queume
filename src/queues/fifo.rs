use std::collections::VecDeque;

use crate::core::{
    config::QueueConfig,
    queue::{Queue, QueueInputError, QueueOutputError},
};

#[derive(Default)]
pub struct FifoQueue<T> {
    queue_config: QueueConfig,
    items: VecDeque<T>,
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
    async fn input(&mut self, item: T) -> Result<(), QueueInputError> {
        if self.queue_config.max_size.is_some_and(|m| self.len() == m) {
            return Err(QueueInputError::MaxSize);
        }

        self.items.push_back(item);

        Ok(())
    }

    async fn output(&mut self) -> Result<T, QueueOutputError> {
        let out_item = self.items.pop_front();

        match out_item {
            Some(i) => Ok(i),
            None => Err(QueueOutputError::Empty),
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
        queue.input("test".to_string()).await.unwrap();

        queue.clear();

        assert_eq!(queue.len(), 0);
    }

    #[tokio::test]
    async fn test_input_adds_item_to_queue() {
        let mut queue: FifoQueue<String> = FifoQueue::default();

        queue.input("test".to_string()).await.unwrap();

        assert_eq!(queue.len(), 1);
    }

    #[tokio::test]
    async fn test_input_after_max_size_errors() {
        let queue_config = QueueConfig { max_size: Some(1) };
        let mut queue: FifoQueue<String> = FifoQueue::new(queue_config);

        queue.input("test".to_string()).await.unwrap();
        let out = queue.input("test2".to_string()).await;

        assert_eq!(queue.len(), 1);
        assert_eq!(out, Err(QueueInputError::MaxSize));
    }

    #[tokio::test]
    async fn test_output_returns_item() {
        let mut queue: FifoQueue<String> = FifoQueue::default();

        queue.input("test".to_string()).await.unwrap();

        let out = queue.output().await.unwrap();

        assert_eq!(out, "test".to_string());
    }

    #[tokio::test]
    async fn test_output_returns_first_item() {
        let mut queue: FifoQueue<String> = FifoQueue::default();

        queue.input("test".to_string()).await.unwrap();
        queue.input("test2".to_string()).await.unwrap();

        let out = queue.output().await.unwrap();

        assert_eq!(out, "test".to_string());
    }

    #[tokio::test]
    async fn test_output_removes_item() {
        let mut queue: FifoQueue<String> = FifoQueue::default();

        queue.input("test".to_string()).await.unwrap();

        let _ = queue.output().await.unwrap();

        assert_eq!(queue.len(), 0);
    }

    #[tokio::test]
    async fn test_output_returns_empty_err() {
        let mut queue: FifoQueue<String> = FifoQueue::default();

        let out = queue.output().await;

        assert_eq!(out, Err(QueueOutputError::Empty));
    }
}
