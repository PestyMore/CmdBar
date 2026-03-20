#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 正确的写法：直接使用 crate 名调用 lib.rs 里的 run() 函数
fn main() {
    cmdbar::run();
}
