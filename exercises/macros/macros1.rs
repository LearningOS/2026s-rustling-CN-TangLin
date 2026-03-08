// 宏定义：符合 Rust 声明式宏的语法规则
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // 关键修改：宏调用必须是 宏名! + 匹配模式（本例中是()）
    my_macro!();
}