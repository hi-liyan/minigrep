use std::{env, process};
use minigrep::{self, Config};

/// 跟着 Bilibili 杨旭老师的《Rust 编程语言入门教程》敲的第一个小实例
///
/// # Example
/// ###
/// ```bash
/// cargo run abc hello.txt
/// ```
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("参数解析错误：{}", err);
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        eprintln!("文件读取错误：{}", err);
        process::exit(1);
    }
}