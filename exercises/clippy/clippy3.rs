// clippy3.rs

fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // 移除无意义且会 panic 的 unwrap()
        // Clippy 提示：避免在已知 None 时调用 unwrap
    }

    // 修复数组元素缺失的逗号（核心语法错误）
    let my_arr = &[
        -1, -2, -3,  // 补充逗号，避免被解析为 -3-4
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 修复 Clippy 禁止的 vec.resize(0, ...) → 改用 clear()
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear(); // 清空 Vec，Clippy 推荐的标准方式
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 正确交换变量（Clippy 提示：避免手动赋值导致交换失败）
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}