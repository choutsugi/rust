use std::{env, error::Error, fs};

// 定义配置
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // 忽略第一个命令行参数
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("query cann't be empty!"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("filename cann't be empty!"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); //读取环境变量

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search_case_sensitive(&config.query, &contents) {
        println!("{}", line)
    }

    Ok(())
}

// 区分大小写：要求完全匹配
pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // 使用迭代器处理
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// 不区分大小写
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    // 使用迭代器处理
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query.as_str()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query, contents)
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
