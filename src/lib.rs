pub mod config;

use std::error::Error;
use std::fs;

use config::Config;


pub fn grep<'a>(config: Config) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in &results {
        println!("{line}");
    }

    Ok(results[0].to_string())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()        
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
    ) -> Vec<&'a str> {

    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

