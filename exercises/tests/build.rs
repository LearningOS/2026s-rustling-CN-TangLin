//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

//! This is the build script for tests7 (and tests8 if needed)
//! 修正后能同时满足 tests7 和 tests8 的需求

fn main() {
    // ========== 处理 tests7 的需求 ==========
    // 获取当前 Unix 时间戳（秒）
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // ✅ 正确格式：cargo:rustc-env=变量名=值
    // 注入 TEST_FOO 环境变量，值为当前时间戳
    let env_command = format!("rustc-env=TEST_FOO={}", timestamp);
    println!("cargo:{}", env_command);

    // ========== 处理 tests8 的需求（如果需要） ==========
    // 启用 "pass" 特性（tests8 所需）
    let feature_command = r#"rustc-cfg=feature="pass""#;
    println!("cargo:{}", feature_command);
}
