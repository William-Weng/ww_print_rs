use std::panic::Location;
use wwprint::ww_print;

fn main() {
    ww_print!("WWPrint Test！");
    test();
}

fn test() {
    ww_print!("test is running!");
}
