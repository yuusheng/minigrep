use crate::error::GrepError;

use clap::Parser;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// String you want to search
    #[arg(index = 1)]
    pub search_string: String,

    /// File path or glob param
    #[arg(index = 2)]
    pub search_file: String,
}

pub struct MiniGrep;
impl MiniGrep {
    pub fn run(args: Args) -> Result<(), GrepError> {
        let search_reg = format!("(?im:{})", &args.search_string.to_string());
        let pattern = Regex::new(&search_reg).unwrap();
        let files: Vec<_> = glob::glob(&args.search_file)?.collect();

        if files.len() == 0 {
            return Err(GrepError::FileNotFound);
        }

        let mut found_pattern = false;
        for file_path in files.into_iter().map(|v| v.unwrap()) {
            let file = File::open(&file_path).unwrap();
            let file_buffer = BufReader::new(file);
            let file_lines = file_buffer.lines().map(|line| line.unwrap());
            for (line_num, line) in file_lines.enumerate() {
                if let Some(_) = pattern.find(&line) {
                    println!("{:?}({line_num}): {line}", file_path);
                    found_pattern = true;
                }
            }
        }

        if !found_pattern {
            return Err(GrepError::PatternNotFound);
        }
        Ok(())
    }
}
