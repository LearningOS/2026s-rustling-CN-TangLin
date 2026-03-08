struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle {width, height}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // 构造矩形，验证宽高与输入一致
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // 替换???为rect.width，检查宽度
        assert_eq!(rect.height, 20); // 替换???为rect.height，检查高度
    }

    // 添加#[should_panic]，验证负宽度触发panic（精准匹配panic消息更严谨）
    #[test]
    #[should_panic(expected = "Rectangle width and height cannot be negative!")]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    // 添加#[should_panic]，验证负高度触发panic
    #[test]
    #[should_panic(expected = "Rectangle width and height cannot be negative!")]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}