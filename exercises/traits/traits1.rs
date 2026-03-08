trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // 实现 append_bar 方法：追加 "Bar" 并返回自身
    fn append_bar(self) -> Self {
        // 方式1：通过 push_str 修改字符串（更直观）
        let mut s = self; // 取得所有权并转为可变
        s.push_str("Bar"); // 追加 "Bar" 到字符串末尾
        s // 返回修改后的字符串

        // 方式2（更简洁）：利用 String 的加法运算符
        // self + "Bar"
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