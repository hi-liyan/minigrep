use std::{env, process};
use minigrep::{self, Config};

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