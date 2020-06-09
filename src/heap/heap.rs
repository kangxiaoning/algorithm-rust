pub struct MaxHeap<T> {
    data: Vec<Option<T>>,
    count: usize,
    capacity: usize,
}

impl<T> MaxHeap<T>
where
    T: Ord + Clone,
{
    pub fn new(capacity: usize) -> Self {
        let data = vec![None; capacity + 1];
        let count = 0;
        Self {
            data,
            count,
            capacity,
        }
    }

    pub fn size(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn shift_up(&mut self, k: usize) {
        let mut k = k;
        while k > 1 && self.data[k / 2] < self.data[k] {
            self.data.swap(k / 2, k);
            k = k / 2;
        }
    }

    pub fn insert(&mut self, item: T) {
        assert!(self.count + 1 <= self.capacity);
        self.data.insert(self.count + 1, Some(item));
        self.count += 1;
        self.shift_up(self.count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut max_heap: MaxHeap<usize> = MaxHeap::new(100);
        max_heap.insert(1);
        max_heap.insert(2);
        max_heap.insert(3);
        assert_eq!(max_heap.size(), 3);
    }
}
