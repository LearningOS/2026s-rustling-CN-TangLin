/*
	queue
	This question requires you to use queues to implement the functionality of the stack
*/


#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T> {
    // 两个队列实现栈：q1 主要存新元素，q2 临时存储迁移的元素
    q1: Queue<T>,
    q2: Queue<T>,
}

impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
        }
    }

    /// 入栈：始终将元素加入 q1
    pub fn push(&mut self, elem: T) {
        self.q1.enqueue(elem);
    }

    /// 出栈：模拟栈的后进先出
    /// 1. 找到非空队列，将前 n-1 个元素迁移到另一个队列
    /// 2. 弹出最后一个元素（栈顶）
    pub fn pop(&mut self) -> Result<T, &str> {
        // 边界：栈空返回错误
        if self.is_empty() {
            return Err("Stack is empty");
        }

        // 区分非空队列（source）和空队列（target）
        let (source, target) = if !self.q1.is_empty() {
            (&mut self.q1, &mut self.q2)
        } else {
            (&mut self.q2, &mut self.q1)
        };

        // 将 source 中前 size-1 个元素迁移到 target
        while source.size() > 1 {
            // 因为 source 非空，unwrap 安全
            let elem = source.dequeue().unwrap();
            target.enqueue(elem);
        }

        // 弹出 source 最后一个元素（栈顶，后进先出）
        source.dequeue()
    }

    /// 栈空判断：两个队列都为空
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = myStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}