// macros3.rs
mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

// 关键：删除手动导入宏的语句，因为 #[macro_export] 已让宏全局可用
// use crate::my_macro;  // 这一行要删掉

fn main() {
    my_macro!();  // 直接使用即可，无需导入
}