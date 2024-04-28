//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

// build.rs
fn main() {
    // 获取当前 UNIX 时间戳
    let timestamp = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .expect("Time went backwards")
    .as_secs();

    // 设置环境变量`TEST_FOO`，值为当前时间戳
    // println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    let command_for_tests7 = format!("rustc-env=TEST_FOO={}", timestamp);
    println!("cargo:{}", command_for_tests7);

    let command_for_tests8 = "rustc-cfg=feature=\"pass\"";
    println!("cargo:{}", command_for_tests8);
}
