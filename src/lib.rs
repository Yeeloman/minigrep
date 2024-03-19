use std::{error::Error, fs, env};
pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        if args.len() < 3 {
            return Err("not enough arguments\nUsage: <query> <file_name>");
        }
        let query: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_name: String = args.next().expect("Didn't get a file name");

        let case_sensitive: bool = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            file_name,
            case_sensitive
        })
    }
}

pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    // use std::io::Read;
    // let mut file: fs::File = fs::File::open(file_name)?;
    // let mut contents: String = String::new();
    // file.read_to_string(&mut contents)?;

    //the above 3 lines can be written as below

    let content: String = fs::read_to_string(&conf.file_name)?;
    let result: Vec<&str> = if conf.case_sensitive {
        search(&conf.query, &content)
    } else {
        search_case_insensitive(&conf.query, &content)
    };
    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut result: Vec<&str> = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         result.push(line);
    //     }
    // }
    // result
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase()
            .contains(&query.to_lowercase()) {
            result.push(line);
        }
    }
    result
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
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
        );
    }
}
