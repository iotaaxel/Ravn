pub mod queue {

    #[derive(Debug)]
    pub struct SensorOutputQueue {
        pub items: Vec<Vec<u32>>,
    }

    #[allow(dead_code)]
    pub(crate) trait Queue {
        type Item;

        fn dequeue(&mut self) -> Option<Self::Item>;
        fn enqueue(&mut self, item: Self::Item);
        fn get(&mut self, index: usize) -> Option<Self::Item>; // Change the return type to Option<&Self::Item>
        fn get_items(&self) -> &[Vec<u32>];
        fn is_empty(&self) -> bool;
        fn new() -> Self;
        fn size(&self) -> usize;
    }

    impl Queue for SensorOutputQueue {
        type Item = Vec<u32>;

        fn dequeue(&mut self) -> Option<Self::Item> {
            //FIFO
            let index = 0;
            if self.is_empty() || !(0..self.items.len()).contains(&index) {
                None
            } else {
                Some(self.items.remove(index))
            }
        }
        fn enqueue(&mut self, item: Self::Item) {
            self.items.push(item);
        }
        fn get(&mut self, index: usize) -> Option<Self::Item> {
            self.items.get(index).cloned()
        }
        fn get_items(&self) -> &[Vec<u32>] {
            &self.items
        }
        fn is_empty(&self) -> bool {
            self.items.is_empty()
        }
        fn new() -> Self {
            SensorOutputQueue { items: vec![] }
        }
        fn size(&self) -> usize {
            self.items.len()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::queue::{Queue, SensorOutputQueue};

    #[test]
    fn test_enqueue_dequeue() {
        let mut queue = SensorOutputQueue::new();
        queue.enqueue(vec![1, 2, 3]);
        queue.enqueue(vec![4, 5, 6]);

        assert_eq!(queue.dequeue(), Some(vec![1, 2, 3]));
        assert_eq!(queue.dequeue(), Some(vec![4, 5, 6]));
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_get() {
        let mut queue = SensorOutputQueue::new();
        queue.enqueue(vec![1, 2, 3]);
        queue.enqueue(vec![4, 5, 6]);

        assert_eq!(queue.get(0), Some(vec![1, 2, 3]));
        assert_eq!(queue.get(1), Some(vec![4, 5, 6]));
        assert_eq!(queue.get(2), None);
    }

    #[test]
    fn test_get_items() {
        let mut queue = SensorOutputQueue::new();
        queue.enqueue(vec![1, 2, 3]);
        queue.enqueue(vec![4, 5, 6]);

        assert_eq!(queue.get_items(), &[vec![1, 2, 3], vec![4, 5, 6]]);
    }

    #[test]
    fn test_is_empty() {
        let mut queue = SensorOutputQueue::new();
        assert!(queue.is_empty());

        queue.enqueue(vec![1, 2, 3]);
        assert!(!queue.is_empty());

        queue.dequeue();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_size() {
        let mut queue = SensorOutputQueue::new();
        assert_eq!(queue.size(), 0);

        queue.enqueue(vec![1, 2, 3]);
        assert_eq!(queue.size(), 1);

        queue.enqueue(vec![4, 5, 6]);
        assert_eq!(queue.size(), 2);

        queue.dequeue();
        assert_eq!(queue.size(), 1);
    }
}
