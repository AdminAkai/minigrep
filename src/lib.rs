use::std::{ fs, error::Error };

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str
}

impl<'a> Config<'a> {
    pub fn build(args: &[String]) -> Result<Config,  &'static str> {
        if args.len() < 3 {
            return Err("Need two arguments");
        }
        let query = &args[1];
        let file_path = &args[2];

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");  
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\n
Rust:
safe, fast, productive.
Pick three.";
        
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
}