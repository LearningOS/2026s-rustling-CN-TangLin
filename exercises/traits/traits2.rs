trait AppendBar {
    fn append_bar(self) -> Self;
}

// 为 Vec<String> 实现 AppendBar trait
impl AppendBar for Vec<String> {
    // 将 self 标记为 mut，因为要修改向量内容
    fn append_bar(mut self) -> Self {
        // 向向量末尾追加 "Bar" 字符串
        self.push(String::from("Bar"));
        // 返回修改后的向量（所有权转移）
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}