/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,          // 堆中元素数量
    items: Vec<T>,         // 存储堆的数组（索引1开始，0为占位）
    comparator: fn(&T, &T) -> bool, // 比较器：最小堆(a<b)，最大堆(a>b)
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()], // 索引0占位，避免计算父节点时越界
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 添加元素到堆，并向上调整维持堆序
    pub fn add(&mut self, value: T) {
        self.count += 1;
        self.items.push(value); // 新元素加到数组末尾
        let mut idx = self.count; // 当前元素索引

        // 向上调整（sift up）：直到父节点不满足交换条件或到根节点
        while idx > 1 {
            let parent_idx = self.parent_idx(idx);
            // 比较当前节点和父节点，若当前更优则交换（最小堆更小，最大堆更大）
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(idx, parent_idx);
                idx = parent_idx; // 继续向上检查父节点
            } else {
                break; // 满足堆序，停止调整
            }
        }
    }

    // 计算父节点索引（索引从1开始）
    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    // 判断当前节点是否有子节点
    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    // 左子节点索引
    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    // 右子节点索引
    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    /// 找到符合堆序的子节点索引（最小堆找更小的，最大堆找更大的）
    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        // 有右子节点：比较左右子节点，返回更优的那个
        if right <= self.count {
            if (self.comparator)(&self.items[right], &self.items[left]) {
                right
            } else {
                left
            }
        } else {
            // 只有左子节点：直接返回左子节点
            left
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// 创建最小堆（比较器：a < b）
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// 创建最大堆（比较器：a > b）
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

/// 实现迭代器：弹出堆顶元素（最小堆弹最小，最大堆弹最大）
impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // 堆空返回None
        if self.is_empty() {
            return None;
        }

        // 1. 取出堆顶元素（索引1）
        let top = std::mem::replace(&mut self.items[1], T::default());
        self.count -= 1;

        // 2. 堆非空时：将最后一个元素移到堆顶，然后向下调整
        if !self.is_empty() {
            // 取出最后一个元素
            let last = self.items.pop().unwrap();
            self.items[1] = last; // 移到堆顶
            let mut idx = 1;

            // 向下调整（sift down）：直到无子女或满足堆序
            while self.children_present(idx) {
                let child_idx = self.smallest_child_idx(idx);
                // 子节点更优则交换
                if (self.comparator)(&self.items[child_idx], &self.items[idx]) {
                    self.items.swap(idx, child_idx);
                    idx = child_idx; // 继续向下检查子节点
                } else {
                    break; // 满足堆序，停止调整
                }
            }
        } else {
            // 堆空：弹出占位的默认值（清理数组）
            self.items.pop();
        }

        Some(top)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}