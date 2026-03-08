pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        // 传入偶数（如4），验证is_even返回true
        assert!(is_even(4));
    }

    #[test]
    fn is_false_when_odd() {
        // 测试题目指定的5，验证is_even返回false
        // 方式1：用!取反，assert!接收true（推荐，简洁）
        assert!(!is_even(5));
        // 方式2：用assert_eq!更直观，等价写法：
        // assert_eq!(is_even(5), false);
    }
}