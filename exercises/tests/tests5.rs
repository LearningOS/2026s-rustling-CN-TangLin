/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
unsafe fn modify_by_address(address: usize) {
    // SAFETY: The function's safety contract guarantees `address` is a valid mutable pointer to a `u32`.
    // We convert the `usize` address to a raw `*mut u32` pointer and dereference it to modify the value,
    // which is safe because the contract ensures the pointer is valid and points to a mutable `u32`.
    unsafe {
        // 将usize地址转换为*mut u32原始指针
        let ptr = address as *mut u32;
        // 解引用指针并修改值为目标值
        *ptr = 0xAABBCCDD;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}