// tests7.rs
//
// When building packages, some dependencies can neither be imported in
// `Cargo.toml` nor be directly linked; some preprocesses varies from code
// generation to set-up package-specific configurations.
//
// Cargo does not aim to replace other build tools, but it does integrate
// with them with custom build scripts called `build.rs`. This file is
// usually placed in the root of the project, while in this case the same
// directory of this exercise.
//
// It can be used to:
//
// - Building a bundled C library.
// - Finding a C library on the host system.
// - Generating a Rust module from a specification.
// - Performing any platform-specific configuration needed for the crate.
//
// When setting up configurations, we can `println!` in the build script
// to tell Cargo to follow some instructions. The generic format is:
//
//     println!("cargo:{}", your_command_in_string);
//
// Please see the official Cargo book about build scripts for more
// information:
// https://doc.rust-lang.org/cargo/reference/build-scripts.html
//
// In this exercise, we look for an environment variable and expect it to
// fall in a range. You can look into the testcase to find out the details.
//
// You should NOT modify this file. Modify `build.rs` in the same directory
// to pass this exercise.
//
// Execute `rustlings hint tests7` or use the `hint` watch subcommand for a
// hint.

fn main() {
    // 1. 获取当前系统时间对应的 Unix 时间戳（秒）
    let current_timestamp = std::time::SystemTime::now()
        // 计算从 Unix 纪元（1970-01-01 00:00:00 UTC）到现在的时长
        .duration_since(std::time::UNIX_EPOCH)
        // 处理时间获取失败的异常（如系统时间异常）
        .expect("Failed to get system time")
        // 转换为秒数（u64 类型，与测试中的 timestamp 类型一致）
        .as_secs();

    // 2. 向 Cargo 发送指令，注入 TEST_FOO 环境变量
    // 格式：cargo:rustc-env=变量名=值（这是 Cargo 识别的设置编译环境变量的标准指令）
    println!("cargo:rustc-env=TEST_FOO={}", current_timestamp);
}
