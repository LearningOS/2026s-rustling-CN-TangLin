// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    // 关键修复：将<?>替换为<&str>，指定Vec的元素类型为字符串切片
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
