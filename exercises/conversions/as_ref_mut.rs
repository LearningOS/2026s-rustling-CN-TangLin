// as_ref_mut.rs
// AsRef 和 AsMut 实现廉价的引用转换

// Obtain the number of bytes (not characters) in the given argument.
// 添加 AsRef<str> trait bound：支持所有能转为 &str 的类型（&str、String 等）
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    // arg.as_ref() 转为 &str → as_bytes() 转为字节切片 → len() 统计字节数
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument.
// 同理添加 AsRef<str> trait bound
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    // arg.as_ref() 转为 &str → chars() 遍历字符 → count() 统计字符数
    arg.as_ref().chars().count()
}

// Squares a number using as_mut().
// 添加 AsMut<u32> trait bound：支持所有能转为 &mut u32 的类型（Box<u32>、&mut u32 等）
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    // 通过 as_mut() 获取 &mut u32 可变引用
    let num_ref = arg.as_mut();
    // 对引用指向的值平方
    *num_ref = *num_ref * *num_ref;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}