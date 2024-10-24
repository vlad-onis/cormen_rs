use std::fmt::Debug;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Fetching elements outside the bounderies of the heap is not allowed")]
    IndexOutOfBounds,

    #[error("Element has no left, it is a leaf")]
    NoLeft,

    #[error("Element has no right")]
    NoRight,
    #[error("The heap is empty")]
    Empty,
}

#[derive(Debug)]
pub enum HeapType {
    MaxHeap,
    MinHeap,
}

#[derive(Debug)]
pub struct BinaryHeap<T> {
    heap: Vec<T>,
    heap_type: HeapType,
}

impl<T> BinaryHeap<T>
where
    T: PartialOrd + Clone + Debug,
{
    pub fn new(heap_type: HeapType, elements: Vec<T>) -> BinaryHeap<T> {
        BinaryHeap {
            heap: elements,
            heap_type,
        }
    }

    pub fn build_heap(&mut self) {
        match self.heap_type {
            HeapType::MinHeap => self.build_min_heap(),
            HeapType::MaxHeap => todo!(),
        }
    }

    fn min_heapify(&mut self, index: usize) -> Result<(), Error> {
        if index >= self.heap.len() {
            return Err(Error::IndexOutOfBounds);
        }

        let left = self.left(index)?;

        // we can unwrap here because if there's no right, there probably is a left
        let right = self.right(index).unwrap_or(index);

        let mut smallest = if self.heap[left] < self.heap[index] {
            left
        } else {
            index
        };

        smallest = if self.heap[right] < self.heap[smallest] {
            right
        } else {
            smallest
        };

        if index != smallest {
            let temp = self.heap[smallest].clone();
            self.heap[smallest] = self.heap[index].clone();
            self.heap[index] = temp;
            return match self.min_heapify(smallest) {
                Ok(_) => Ok(()),
                Err(Error::NoLeft) => Ok(()),
                Err(Error::NoRight) => Ok(()),
                Err(e) => Err(e),
            };
        }

        Ok(())
    }

    fn build_min_heap(&mut self) {
        let heap_size = self.heap.len();

        if heap_size > 1 {
            for index in (0..heap_size / 2).rev() {
                let r = self.min_heapify(index);
                println!("Minheapify for element at {index} finished: {r:?}");
            }
        }
    }

    pub fn parent(&self, index: usize) -> Result<usize, Error> {
        if index >= self.heap.len() || index == 0 {
            return Err(Error::IndexOutOfBounds);
        }

        let parent_index = ((index + 1) / 2) - 1;

        Ok(parent_index)
    }

    pub fn left(&self, index: usize) -> Result<usize, Error> {
        let left_element_index = (index * 2) + 1;

        if left_element_index >= self.heap.len() {
            return Err(Error::NoLeft);
        }

        Ok(left_element_index)
    }

    pub fn right(&self, index: usize) -> Result<usize, Error> {
        let right_element_index = (index * 2) + 2;

        if right_element_index >= self.heap.len() {
            return Err(Error::NoRight);
        }

        Ok(right_element_index)
    }

    pub fn minimum(&self) -> Option<&T> {
        if self.heap.is_empty() {
            None
        } else {
            Some(&self.heap[0])
        }
    }

    pub fn extract_min(&mut self) -> Option<T> {
        if self.heap.len() == 1 {
            let minimum = self.heap[0].clone();
            self.heap.remove(0);
            return Some(minimum);
        }

        match self.minimum().cloned() {
            None => None,
            Some(minimum) => {
                let last_index = self.heap.len() - 1;
                self.heap[0] = self.heap[last_index].clone();
                self.heap.remove(last_index);

                let res = self.min_heapify(0);
                println!("Min heapify after extraction finished with: {res:?}");

                Some(minimum)
            }
        }
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    pub fn valid_parent() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let bin_heap = BinaryHeap::new(HeapType::MinHeap, arr);

        let parent = bin_heap.parent(9).unwrap();
        assert_eq!(parent, 4);

        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        let bin_heap = BinaryHeap::new(HeapType::MinHeap, arr);

        let parent = bin_heap.parent(8).unwrap();
        assert_eq!(parent, 3);

        let parent = bin_heap.parent(2).unwrap();
        assert_eq!(parent, 0);

        let parent = bin_heap.parent(5).unwrap();
        assert_eq!(parent, 2);
    }

    #[test]
    pub fn invalid_parent() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let bin_heap = BinaryHeap::new(HeapType::MinHeap, arr);

        let parent = bin_heap.parent(10);
        assert!(parent.is_err());

        let parent = bin_heap.parent(0);
        assert!(parent.is_err());
    }

    #[test]
    pub fn valid_left_right() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let bin_heap = BinaryHeap::new(HeapType::MinHeap, arr);

        let left = bin_heap.left(3).unwrap();
        assert_eq!(left, 7);

        let right = bin_heap.right(3).unwrap();
        assert_eq!(right, 8);
    }

    #[test]
    pub fn invalid_left_right() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let bin_heap = BinaryHeap::new(HeapType::MinHeap, arr);

        let left = bin_heap.left(6);
        assert!(left.is_err());

        let right = bin_heap.right(4);
        assert!(right.is_err());
    }

    #[test]
    pub fn build_min_heap() {
        let arr = vec![7, 6, 5, 4, 3, 2, 1];
        let mut bin_heap = BinaryHeap::new(HeapType::MinHeap, arr);
        bin_heap.build_heap();
        assert_eq!(bin_heap.heap, vec![1, 3, 2, 4, 6, 7, 5]);

        let arr = vec![8, 7, 6, 5, 4, 3, 2, 1];
        let mut bin_heap = BinaryHeap::new(HeapType::MinHeap, arr);
        bin_heap.build_heap();
        assert_eq!(bin_heap.heap, vec![1, 4, 2, 5, 8, 3, 6, 7]);

        let arr = vec![2, 1];
        let mut bin_heap = BinaryHeap::new(HeapType::MinHeap, arr);
        bin_heap.build_heap();
        assert_eq!(bin_heap.heap, vec![1, 2]);
    }

    #[test]
    pub fn minimum_element() {
        let arr = vec![1, 12, 14, 6, 18, 112, 44, 32];
        let mut bin_heap = BinaryHeap::new(HeapType::MinHeap, arr);
        bin_heap.build_heap();

        assert_eq!(bin_heap.minimum().cloned().unwrap(), 1);
    }

    #[test]
    pub fn extract_minimum_one_element() {
        let arr = vec![1];
        let mut bin_heap = BinaryHeap::new(HeapType::MinHeap, arr);
        bin_heap.build_heap();

        assert_eq!(bin_heap.extract_min().unwrap(), 1);
    }

    #[test]
    pub fn extract_minimum_no_elements() {
        let arr: Vec<u32> = vec![];
        let mut bin_heap = BinaryHeap::new(HeapType::MinHeap, arr);
        bin_heap.build_heap();

        assert!(bin_heap.extract_min().is_none());
    }

    #[test]
    pub fn extract_minimum() {
        let arr = vec![1, 12, 14, 6, 18, 112, 44, 32];
        let mut bin_heap = BinaryHeap::new(HeapType::MinHeap, arr);
        bin_heap.build_heap();

        assert_eq!(bin_heap.extract_min().unwrap(), 1);
        assert_eq!(bin_heap.heap, vec![6, 12, 14, 32, 18, 112, 44]);

        assert_eq!(bin_heap.extract_min().unwrap(), 6);
        assert_eq!(bin_heap.heap, vec![12, 18, 14, 32, 44, 112]);

        assert_eq!(bin_heap.extract_min().unwrap(), 12);
        assert_eq!(bin_heap.heap, vec![14, 18, 112, 32, 44]);

        assert_eq!(bin_heap.extract_min().unwrap(), 14);
        assert_eq!(bin_heap.heap, vec![18, 32, 112, 44]);

        assert_eq!(bin_heap.extract_min().unwrap(), 18);
        assert_eq!(bin_heap.heap, vec![32, 44, 112]);

        assert_eq!(bin_heap.extract_min().unwrap(), 32);
        assert_eq!(bin_heap.heap, vec![44, 112]);

        assert_eq!(bin_heap.extract_min().unwrap(), 44);
        assert_eq!(bin_heap.heap, vec![112]);

        assert_eq!(bin_heap.extract_min().unwrap(), 112);
        assert_eq!(bin_heap.heap, vec![]);

        assert!(bin_heap.extract_min().is_none());
    }
}
