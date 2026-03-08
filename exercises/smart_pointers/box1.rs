#[derive(PartialEq, Debug)]
pub enum List {
    // 关键修改：用 Box<List> 替代直接的 List，解决递归类型大小计算问题
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

// 创建空的 cons list（仅返回 Nil）
pub fn create_empty_list() -> List {
    List::Nil
}

// 创建非空的 cons list（示例：1 -> Nil）
pub fn create_non_empty_list() -> List {
    List::Cons(1, Box::new(List::Nil))
    // 也可以创建更复杂的链表，比如：
    // List::Cons(5, Box::new(List::Cons(3, Box::new(List::Nil))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}