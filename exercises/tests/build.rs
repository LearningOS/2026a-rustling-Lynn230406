//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // 获取当前系统时间戳
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); 
    
    // 1. 针对 tests7：设置环境变量 TEST_FOO
    let your_command_1 = format!("rustc-env=TEST_FOO={}", timestamp);
    println!("cargo:{}", your_command_1);

    // 2. 针对 tests8：开启 "pass" 编译特征
    let your_command_2 = "rustc-cfg=feature=\"pass\"";
    println!("cargo:{}", your_command_2);
}