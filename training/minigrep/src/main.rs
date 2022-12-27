use minigrep::{run, Config};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("failed to parse params with err: {}", err);
        process::exit(1)
    });

    if let Err(err) = run(config) {
        eprintln!("应用错误：{}", err);
        process::exit(1)
    }
}
