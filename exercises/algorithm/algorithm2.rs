/*
	double linked list reverse
	This problem requires you to reverse a doubly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::Vec;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
    // 新增：存储所有节点的 Box，自动管理内存，避免泄漏
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
            nodes: Vec::new(),
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.end;
        
        // 安全获取指针：先存入 Vec 再取引用，避免 raw pointer 泄漏
        let node_ptr = NonNull::from(&*node);
        let node_ptr_opt = Some(node_ptr);

        match self.end {
            None => self.start = node_ptr_opt,
            Some(end_ptr) => unsafe {
                (*end_ptr.as_ptr()).next = node_ptr_opt;
            },
        }
        
        self.nodes.push(node); // 节点所有权交给 Vec，自动释放
        self.end = node_ptr_opt;
        self.length += 1;
    }

    // 修复：只读操作改为 &self，添加索引合法性检查
    pub fn get(&self, index: i32) -> Option<&T> {
        // 处理索引越界（负数/超过长度）
        if index < 0 || index as u32 >= self.length {
            return None;
        }
        self.get_ith_node(self.start, index)
    }

    // 修复：改为 &self，无需可变引用
    fn get_ith_node(&self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    pub fn reverse(&mut self) {
        // 边界处理：空链表/单节点链表直接返回
        if self.length <= 1 {
            return;
        }

        let mut current = self.start;
        let mut temp: Option<NonNull<Node<T>>> = None;

        while let Some(cur_ptr) = current {
            unsafe {
                // 交换 next 和 prev 指针
                temp = (*cur_ptr.as_ptr()).next;
                (*cur_ptr.as_ptr()).next = (*cur_ptr.as_ptr()).prev;
                (*cur_ptr.as_ptr()).prev = temp;
                // 移动到下一个节点（原 prev 方向）
                current = temp;
            }
        }

        // 交换头尾指针
        std::mem::swap(&mut self.start, &mut self.end);
    }
}

// 修复：手动遍历节点，避免多余逗号，输出更规范
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
        // 新增：测试越界访问
        assert_eq!(None, list.get(3));
        assert_eq!(None, list.get(-1));
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
    fn test_reverse_linked_list_1() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![2, 3, 5, 11, 9, 7];
        let reverse_vec = vec![7, 9, 11, 5, 3, 2];
        
        for &num in &original_vec {
            list.add(num);
        }
        
        println!("Original Linked List is [{}]", list);
        list.reverse();
        println!("Reversed Linked List is [{}]", list);
        
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
        // 验证长度不变
        assert_eq!(original_vec.len() as u32, list.length);
    }

    #[test]
    fn test_reverse_linked_list_2() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![34, 56, 78, 25, 90, 10, 19, 34, 21, 45];
        let reverse_vec = vec![45, 21, 34, 19, 10, 90, 25, 78, 56, 34];
        
        for &num in &original_vec {
            list.add(num);
        }
        
        println!("Original Linked List is [{}]", list);
        list.reverse();
        println!("Reversed Linked List is [{}]", list);
        
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
        assert_eq!(original_vec.len() as u32, list.length);
    }

    // 新增：测试空链表/单节点链表反转（边界场景）
    #[test]
    fn test_reverse_empty_or_single_node() {
        // 空链表
        let mut empty_list = LinkedList::<i32>::new();
        empty_list.reverse();
        assert_eq!(0, empty_list.length);
        assert_eq!(None, empty_list.get(0));

        // 单节点链表
        let mut single_node_list = LinkedList::<i32>::new();
        single_node_list.add(100);
        single_node_list.reverse();
        assert_eq!(1, single_node_list.length);
        assert_eq!(Some(&100), single_node_list.get(0));
    }
}