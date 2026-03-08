// move_semantics2.rs
//
// Expected output:
// vec0 has length 3, with contents `[22, 44, 66]`
// vec1 has length 4, with contents `[22, 44, 66, 88]`
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let mut vec0 = Vec::new();
    // 先给 vec0 添加元素，使其拥有预期内容
    vec0.push(22);
    vec0.push(44);
    vec0.push(66);

    let mut vec1 = fill_vec(vec0.clone());

    println!(
        "{} has length {}, with contents: `{:?}`",
        "vec0",
        vec0.len(),
        vec0
    );

    vec1.push(88);

    println!(
        "{} has length {}, with contents `{:?}`",
        "vec1",
        vec1.len(),
        vec1
    );
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    // 移除重复添加的元素（否则 vec1 会有重复值）
    vec
}
