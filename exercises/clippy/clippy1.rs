// 移除无用的 use std::f32;，并导入标准库的 PI 常量（可选：直接使用完整路径）
use std::f32::consts::PI;

fn main() {
    // 关键修改：使用标准库的 PI 常量，替代手动硬编码的 3.14f32
    let pi = PI;
    let radius = 5.00f32;

    // 可选优化：简化为 radius.powi(2)（等价于原写法，但更简洁）
    let area = pi * radius.powi(2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    );
}