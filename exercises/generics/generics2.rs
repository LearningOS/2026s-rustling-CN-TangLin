// 1. 为结构体添加泛型参数 T，将 value 类型改为 T
struct Wrapper<T> {
    value: T,
}

// 2. 为 impl 块添加泛型参数 T，匹配结构体的泛型
impl<T> Wrapper<T> {
    // 3. new 方法的参数类型改为 T，返回 Self（即 Wrapper<T>）
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}