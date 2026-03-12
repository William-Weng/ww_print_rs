#[macro_export]
macro_rules! ww_print {

    ($($argument:tt)*) => {{

        let location = Location::caller();
        let short_file = location.file();
        let message = format!($($argument)*);
        
        // 第一行：藍底黃字，顯示文件名和行號
        println!("\x1b[44m\x1b[30m[{}:{}]\x1b[0m", short_file, location.line());

        // 第二行：正常顯示消息
        println!("{}", message);
        
        // 第三行：空行
        println!("");
    }};
}
