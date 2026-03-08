// try_from_into.rs
//
// TryFrom is a simple and safe type conversion that may fail in a controlled
// way under some circumstances. Basically, this is the same as From. The main
// difference is that this should return a Result type instead of the target
// type itself. You can read more about it at
// https://doc.rust-lang.org/std/convert/trait.TryFrom.html
//
// Execute `rustlings hint try_from_into` or use the `hint` watch subcommand for
// a hint.

use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// We will use this error type for these `TryFrom` conversions.
#[derive(Debug, PartialEq)]
enum IntoColorError {
    // Incorrect length of slice
    BadLen,
    // Integer conversion error
    IntConversion,
}

// 辅助函数：检查i16是否在u8范围内（0..=255），转换失败返回IntConversion
fn convert_i16_to_u8(n: i16) -> Result<u8, IntoColorError> {
    if (0..=255).contains(&n) {
        Ok(n as u8)
    } else {
        Err(IntoColorError::IntConversion)
    }
}

// Tuple implementation (三元组实现)
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;
    
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        // 解构三元组为红、绿、蓝三个分量
        let (r, g, b) = tuple;
        // 逐个转换并检查范围，? 操作符传递错误
        let red = convert_i16_to_u8(r)?;
        let green = convert_i16_to_u8(g)?;
        let blue = convert_i16_to_u8(b)?;
        
        Ok(Color { red, green, blue })
    }
}

// Array implementation (固定长度数组实现)
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;
    
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        // 解构数组为三个分量（数组长度固定为3，无需检查长度）
        let [r, g, b] = arr;
        let red = convert_i16_to_u8(r)?;
        let green = convert_i16_to_u8(g)?;
        let blue = convert_i16_to_u8(b)?;
        
        Ok(Color { red, green, blue })
    }
}

// Slice implementation (切片实现)
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;
    
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        // 第一步：检查切片长度是否为3，否则返回BadLen错误
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        }
        
        // 第二步：提取三个分量并转换
        let r = slice[0];
        let g = slice[1];
        let b = slice[2];
        let red = convert_i16_to_u8(r)?;
        let green = convert_i16_to_u8(g)?;
        let blue = convert_i16_to_u8(b)?;
        
        Ok(Color { red, green, blue })
    }
}

fn main() {
    // Use the `try_from` function
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1); // 输出：Ok(Color { red: 183, green: 65, blue: 14 })

    // Since TryFrom is implemented for Color, we should be able to use TryInto
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2); // 输出：Ok(Color { red: 183, green: 65, blue: 14 })

    let v = vec![183, 65, 14];
    // With slice we should use `try_from` function
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3); // 输出：Ok(Color { red: 183, green: 65, blue: 14 })
    // or take slice within round brackets and use TryInto
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4); // 输出：Ok(Color { red: 183, green: 65, blue: 14 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_out_of_range_positive() {
        assert_eq!(
            Color::try_from((256, 1000, 10000)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_out_of_range_negative() {
        assert_eq!(
            Color::try_from((-1, -10, -256)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_sum() {
        assert_eq!(
            Color::try_from((-1, 255, 255)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_correct() {
        let c: Result<Color, _> = (183, 65, 14).try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_array_out_of_range_positive() {
        let c: Result<Color, _> = [1000, 10000, 256].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_out_of_range_negative() {
        let c: Result<Color, _> = [-10, -256, -1].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_sum() {
        let c: Result<Color, _> = [-1, 255, 255].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_correct() {
        let c: Result<Color, _> = [183, 65, 14].try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_sum() {
        let arr = [-1, 255, 255];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, _> = Color::try_from(&v[..]);
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
    #[test]
    fn test_slice_insufficient_length() {
        let v = vec![0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
}