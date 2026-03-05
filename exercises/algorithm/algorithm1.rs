/*
    single linked list merge
    This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

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
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        // 安全地创建 NonNull 指针（Box::into_raw 不会返回 null）
        let node_ptr = NonNull::new(Box::into_raw(node)).unwrap();
        
        match self.end {
            None => self.start = Some(node_ptr),
            Some(end_ptr) => unsafe {
                // 将新节点挂到尾节点的 next 上
                (*end_ptr.as_ptr()).next = Some(node_ptr);
            },
        }
        self.end = Some(node_ptr);
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        if index < 0 || index as u32 >= self.length {
            return None; // 处理索引越界
        }
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
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
        T: Ord,
    {
        let mut merged_list = LinkedList::new();
        let mut ptr_a = list_a.start;
        let mut ptr_b = list_b.start;

        // 核心修复：合并时不再截断节点的 next 指针
        while let (Some(a), Some(b)) = (ptr_a, ptr_b) {
            let val_a = unsafe { &(*a.as_ptr()).val };
            let val_b = unsafe { &(*b.as_ptr()).val };

            if val_a <= val_b {
                // 取出当前 a 节点，并更新 ptr_a 为下一个节点
                merged_list.append_node(a);
                ptr_a = unsafe { (*a.as_ptr()).next };
            } else {
                // 取出当前 b 节点，并更新 ptr_b 为下一个节点
                merged_list.append_node(b);
                ptr_b = unsafe { (*b.as_ptr()).next };
            }
        }

        // 处理 list_a 剩余的节点
        while let Some(a) = ptr_a {
            merged_list.append_node(a);
            ptr_a = unsafe { (*a.as_ptr()).next };
        }

        // 处理 list_b 剩余的节点
        while let Some(b) = ptr_b {
            merged_list.append_node(b);
            ptr_b = unsafe { (*b.as_ptr()).next };
        }

        merged_list.length = list_a.length + list_b.length;
        merged_list
    }

    // 修复：不再清空节点的 next 指针，只处理链表的头尾指针
    fn append_node(&mut self, node_ptr: NonNull<Node<T>>) {
        let node_ptr_opt = Some(node_ptr);
        
        match self.end {
            None => {
                // 合并链表为空时，start 和 end 都指向当前节点
                self.start = node_ptr_opt;
                self.end = node_ptr_opt;
            }
            Some(end_ptr) => unsafe {
                // 将当前节点挂到合并链表尾节点的 next 上
                (*end_ptr.as_ptr()).next = node_ptr_opt;
                // 更新合并链表的尾指针为当前节点
                self.end = node_ptr_opt;
            },
        }
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
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
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
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
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
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
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
}