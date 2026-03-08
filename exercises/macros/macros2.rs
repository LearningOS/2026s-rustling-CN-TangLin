// 关键修改：将宏定义移到调用它的代码（main函数）之前
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}