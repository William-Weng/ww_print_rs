# ww_print_rs
WWPrint for Rust - Debug print with file/line info.

### Introduction
- WWPrint for Rust - Debug print with file/line info.
- 一個在Rust開發時除錯，用能顯示檔案名稱及行數的print巨集功能。

### Example
```rust
use std::panic::Location;
use wwprint::ww_print;

fn main() {
    ww_print!("WWPrint Test！");
    test();
}

fn test() {
    ww_print!("test is running!");
}
```