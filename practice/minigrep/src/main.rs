use minigrep::{run, Config};
use std::{env, process};

fn main() {
    // env::args()实现了迭代器
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("failed to parse params with err: {}", err);
        process::exit(1)
    });

    if let Err(err) = run(config) {
        eprintln!("应用错误：{}", err);
        process::exit(1)
    }
}
