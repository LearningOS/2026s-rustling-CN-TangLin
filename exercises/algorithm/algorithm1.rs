/*
    single linked list merge
    This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::mem;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
    // 新增：存储所有节点的 Box，避免内存泄漏
    nodes: Vec<Box<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
            nodes: Vec::new(), // 初始化节点存储容器
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        // 先将节点存入 Vec，再获取 NonNull 指针（避免内存泄漏）
        let node_ptr = NonNull::from(&*node);
        self.nodes.push(node);
        
        match self.end {
            None => self.start = Some(node_ptr),
            Some(end_ptr) => unsafe {
                (*end_ptr.as_ptr()).next = Some(node_ptr);
            },
        }
        self.end = Some(node_ptr);
        self.length += 1;
    }

    pub fn get(&self, index: i32) -> Option<&T> {
        if index < 0 || index as u32 >= self.length {
            return None; // 处理索引越界
        }
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self
    where
        T: Ord + Clone, // 新增 Clone 约束，避免所有权转移问题
    {
        let mut merged_list = LinkedList::new();
        let mut ptr_a = list_a.start;
        let mut ptr_b = list_b.start;

        // 核心逻辑：按序合并两个链表的节点
        while let (Some(a), Some(b)) = (ptr_a, ptr_b) {
            let val_a = unsafe { &(*a.as_ptr()).val };
            let val_b = unsafe { &(*b.as_ptr()).val };

            if val_a <= val_b {
                // 克隆节点值添加到合并链表（避免原链表节点所有权问题）
                merged_list.add(val_a.clone());
                ptr_a = unsafe { (*a.as_ptr()).next };
            } else {
                merged_list.add(val_b.clone());
                ptr_b = unsafe { (*b.as_ptr()).next };
            }
        }

        // 处理 list_a 剩余的节点
        while let Some(a) = ptr_a {
            merged_list.add(unsafe { (*a.as_ptr()).val.clone() });
            ptr_a = unsafe { (*a.as_ptr()).next };
        }

        // 处理 list_b 剩余的节点
        while let Some(b) = ptr_b {
            merged_list.add(unsafe { (*b.as_ptr()).val.clone() });
            ptr_b = unsafe { (*b.as_ptr()).next };
        }

        // 精准计算长度（不再依赖原链表长度相加）
        merged_list.length = merged_list.nodes.len() as u32;
        merged_list
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut current = self.start;
        let mut first = true;
        while let Some(node) = current {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}", unsafe { &(*node.as_ptr()).val })?;
            current = unsafe { (*node.as_ptr()).next };
            first = false;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is [{}]", list);
        assert_eq!(3, list.length);
        assert_eq!(Some(&1), list.get(0));
        assert_eq!(Some(&2), list.get(1));
        assert_eq!(Some(&3), list.get(2));
        assert_eq!(None, list.get(3)); // 测试越界
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is [{}]", list_str);
        assert_eq!(3, list_str.length);
        assert_eq!(Some(&"A".to_string()), list_str.get(0));
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1,3,5,7];
        let vec_b = vec![2,4,6,8];
        let target_vec = vec![1,2,3,4,5,6,7,8];
        
        for &num in &vec_a {
            list_a.add(num);
        }
        for &num in &vec_b {
            list_b.add(num);
        }
        println!("list a [{}], list b [{}]", list_a, list_b);
        let list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is [{}]", list_c);
        
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
        assert_eq!(8, list_c.length); // 验证长度正确
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11,33,44,88,89,90,100];
        let vec_b = vec![1,22,30,45];
        let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

        for &num in &vec_a {
            list_a.add(num);
        }
        for &num in &vec_b {
            list_b.add(num);
        }
        println!("list a [{}], list b [{}]", list_a, list_b);
        let list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is [{}]", list_c);
        
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
        assert_eq!(11, list_c.length); // 验证长度正确
    }
}