// 关键修改：给模块添加 #[macro_use]，导出内部的宏到外部作用域
#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!(); // 现在能找到模块内导出的宏
}