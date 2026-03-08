fn main() {
    let mut res = 42;
    let option = Some(12);
    
    // 关键修改：用 if let 替代冗余的 for 循环（Clippy 推荐的规范写法）
    if let Some(x) = option {
        res += x;
    }

    // 备选方案（更简洁，适合简单逻辑）：
    // res += option.unwrap_or(0);

    println!("{}", res); // 输出 54，与原逻辑一致
}