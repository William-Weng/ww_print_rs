# WWPrint
![Rust-2021](https://img.shields.io/badge/rust-2021-orange.svg?style=flat) ![TAG](https://img.shields.io/github/v/tag/William-Weng/ww_print_rs) ![LICENSE](https://img.shields.io/badge/LICENSE-MIT-yellow.svg?style=flat)

### Introduction
- WWPrint for Rust - Debug print with file/line info.
- 一個在Rust開發時除錯，用能顯示檔案名稱及行數的print巨集功能。

<img width="745" height="254" alt="Example" src="https://github.com/user-attachments/assets/9d3c5972-7346-4046-93d3-4fc506b4ebf8" />

### [Install](https://crates.io/crates/wwprint)
```bash
cargo add wwprint
```

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
