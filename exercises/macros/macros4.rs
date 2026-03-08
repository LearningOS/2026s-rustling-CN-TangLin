#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    }; // 关键修改：添加分号分隔分支
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }; // 关键修改：最后一个分支也建议加结束分号
}

fn main() {
    my_macro!();
    my_macro!(7777);
}