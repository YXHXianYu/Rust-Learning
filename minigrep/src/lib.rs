use std::error::Error;
use std::fs;
use std::any::type_name;

pub struct Config {
    pub target: String,
    pub file_path: String,
    pub is_ignore_case: bool,
}

impl Config {
    pub fn build<T: Iterator<Item = String>>(
        mut args: T
    ) -> Result<Config, &'static str> {
        args.next();

        let target = match args.next() {
            Some(arg) => arg,
            None => return Err("Please input `target_string` and `file_path` as arguments"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Please input `target_string` and `file_path` as arguments"),
        };

        Ok(Config {
            target,
            file_path,
            is_ignore_case: false,
        })
    }
    
    pub fn set_is_ignore_case(mut self, value: bool) -> Config {
        self.is_ignore_case = value;
        self
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    for line in search(&config, &contents) {
        println!("{line}");
    }

    Ok(())
}

#[allow(dead_code)]
fn print_type_name<T>(_: &T) {
    eprintln!("Type: {}", type_name::<T>());
}

fn search<'a>(config: &Config, contents: &'a str, ) -> Vec<&'a str> {
    let target = if config.is_ignore_case {
        &config.target.to_lowercase()
    } else {
        &config.target
    };

    contents
        .lines()
        .filter(|line| {
            let line_converted = if config.is_ignore_case {
                line.to_lowercase()
            } else {
                line.to_string()
            };
            line_converted.contains(target)
        })
        .collect()
}

// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search1() {
        let config = Config {
            target: "duct".to_string(),
            file_path: "".to_string(),
            is_ignore_case: false,
        };
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(&config, contents));
    }

    #[test]
    fn search_ignore_case1() {
        let config = Config {
            target: "to".to_string(),
            file_path: "".to_string(),
            is_ignore_case: false,
        };
        let contents = "\
to
To
tO
TO
not
OT";

        assert_eq!(vec!["to"], search(&config, &contents));
    }

    #[test]
    fn search_ignore_case2() {
        let config = Config {
            target: "to".to_string(),
            file_path: "".to_string(),
            is_ignore_case: true,
        };
        let contents = "\
to
To
tO
TO
not
OT";

        assert_eq!(vec!["to", "To", "tO", "TO"], search(&config, &contents));
    }
}
