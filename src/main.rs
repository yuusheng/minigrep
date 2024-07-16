mod error;

use clap::Parser;
use error::GrepError;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(index = 1)]
    search_string: String,

    #[arg(index = 2)]
    search_file: String,
}

fn main() -> Result<(), GrepError> {
    let args: Args = Args::parse();
    let search_reg = format!("(?im:{})", &args.search_string.to_string());
    let pattern = Regex::new(&search_reg).unwrap();
    let files: Vec<_> = glob::glob(&args.search_file)?.collect();

    if files.len() == 0 {
        return Err(GrepError::FileNotFound);
    }

    files.into_iter().map(|v| v.unwrap()).for_each(|filepath| {
        let file = File::open(filepath).unwrap();
        let file_buffer = BufReader::new(file);
        for line in file_buffer.lines() {
            if let Ok(line) = line {
                if let Some(_) = pattern.find(&line) {
                    println!("{line}");
                }
            }
        }
    });

    Ok(())
}
