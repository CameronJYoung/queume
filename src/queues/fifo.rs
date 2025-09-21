use std::collections::VecDeque;

use crate::core::queue::{Queue, QueueInputError, QueueOutputError};

pub struct FifoQueue<T> {
    items: VecDeque<T>,
}

impl<T> FifoQueue<T> {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

#[async_trait::async_trait]
impl<T: Send> Queue<T> for FifoQueue<T> {
    async fn input(&mut self, item: T) -> Result<(), QueueInputError> {
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
    async fn test_input_adds_item_to_queue() {
        let mut queue: FifoQueue<String> = FifoQueue::new();

        queue.input("test".to_string()).await.unwrap();

        assert_eq!(queue.len(), 1);
    }

    #[tokio::test]
    async fn test_output_returns_item() {
        let mut queue: FifoQueue<String> = FifoQueue::new();

        queue.input("test".to_string()).await.unwrap();

        let out = queue.output().await.unwrap();

        assert_eq!(out, "test".to_string());
    }

    #[tokio::test]
    async fn test_output_returns_first_item() {
        let mut queue: FifoQueue<String> = FifoQueue::new();

        queue.input("test".to_string()).await.unwrap();
        queue.input("test2".to_string()).await.unwrap();

        let out = queue.output().await.unwrap();

        assert_eq!(out, "test".to_string());
    }

    #[tokio::test]
    async fn test_output_removes_item() {
        let mut queue: FifoQueue<String> = FifoQueue::new();

        queue.input("test".to_string()).await.unwrap();

        let _ = queue.output().await.unwrap();

        assert_eq!(queue.len(), 0);
    }

    #[tokio::test]
    async fn test_output_returns_empty_err() {
        let mut queue: FifoQueue<String> = FifoQueue::new();

        let out = queue.output().await;

        assert_eq!(out, Err(QueueOutputError::Empty));
    }
}
