use std::{error::Error, fs, env::{self, Args}};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let result = if config.case_insensitive {
        search_case_sensitive(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    if result.len() > 0 {
        for line in result {
            println!("{}", line);
        }
    } else {
        println!("没有找到！");
    }
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("没有足够的参数");
        }

        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("没有获取到要查询的关键字")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("没有获取到文件名称")
        };

        Ok(Config {
            query,
            filename,
            case_insensitive: env::var("CASE_INSENSITIVE").is_err(),
        })
    }
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut result = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         result.push(line);
    //     }
    // }
    // result
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut result = Vec::new();
    // let query = query.to_lowercase();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         result.push(line)
    //     }
    // }
    // result
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "Rust";
        let contents = "\
        Rust is
        Safe, Fast, Productive.";
        assert_eq!(vec!["Rust is"], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rust";
        let contents = "\
        Rust is
        Safe, Fast, Productive.";
        assert_eq!(vec!["Rust is"], search_case_insensitive(query, contents));
    }
}