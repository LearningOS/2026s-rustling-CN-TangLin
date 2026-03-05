trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // 实现 append_bar 方法：拼接 "Bar" 并返回新字符串
    fn append_bar(self) -> Self {
        // self 是 String 类型，拼接 "Bar" 后返回新的 String
        self + "Bar"
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s); // 输出：s: FooBar
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}