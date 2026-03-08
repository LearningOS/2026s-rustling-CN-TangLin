//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // ========== 处理 tests7：设置环境变量 TEST_FOO ==========
    // 获取当前系统时间的 UNIX 时间戳（秒）
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    // 正确的 Cargo 指令：设置环境变量 TEST_FOO
    let test7_command = format!("rustc-env=TEST_FOO={}", timestamp);
    println!("cargo:{}", test7_command);

    // ========== 处理 tests8：启用 "pass" 特性 ==========
    // 正确的 Cargo 指令：启用名为 "pass" 的特性
    let test8_command = r#"rustc-cfg=feature="pass""#;
    println!("cargo:{}", test8_command);
}