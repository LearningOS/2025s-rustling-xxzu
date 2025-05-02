//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

// fn main() {
//     // In tests7, we should set up an environment variable
//     // called `TEST_FOO`. Print in the standard output to let
//     // Cargo do it.
//     let timestamp = std::time::SystemTime::now()
//         .duration_since(std::time::UNIX_EPOCH)
//         .unwrap()
//         .as_secs(); // What's the use of this timestamp here?
//                     // let your_command = format!(
//                     //     "Your command here with {}, please checkout exercises/tests/build.rs",
//                     //     timestamp
//     println!("cargo:rerun-if-changed=build.rs"); // 确保 build.rs 文件变更时重新编译
//     println!("cargo:env=TEST_FOO={}", timestamp); // 设置环境变量

//     // 启用 pass 特性
//     println!("cargo:enable-pass-feature=true"); // 启用 pass 特性

//     // println!("cargo:rerun-if-changed=build.rs"); // 确保 build.rs 文件变更时重新编译
//     // println!("cargo:env=TEST_FOO={}", timestamp); // 设置环境变量

//     // In tests8, we should enable "pass" feature to make the
//     // testcase return early. Fill in the command to tell
//     // Cargo about that.
//     let your_command = "Your command here, please checkout exercises/tests/build.rs";
//     println!("cargo:{}", your_command);
// }

fn main() {
    // 当前 UNIX 时间戳
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 设置环境变量 TEST_FOO
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 如果 build.rs 有修改，重新运行
    println!("cargo:rerun-if-changed=build.rs");

    // 启用 pass 配置（用于 tests8）
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
