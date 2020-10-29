use scanner::Scanner;
use std::cmp::Ordering;
use std::env;
use std::error::Error;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::process;

mod error;
mod scanner;
mod token;
pub enum RunType {
    File(String),
    Prompt,
}

impl RunType {
    pub fn new(args: &[String]) -> Result<RunType, &'static str> {
        // first argument is always name of the binary
        match args.len().cmp(&2) {
            Ordering::Greater => Err("Usage: loxr [script]"),
            Ordering::Less => Ok(RunType::Prompt),
            Ordering::Equal => {
                let path = args[1].clone();
                Ok(RunType::File(path))
            }
        }
    }
}

pub fn run_file(path: &str) {
    let file_contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    if let Err(error) = run(file_contents) {
        println!("Error: {}", error);
        process::exit(1);
    };
}

pub fn run_prompt() {
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut line = String::new();
        let bytes_read = stdin().read_line(&mut line).expect("failed to read line");
        if bytes_read == 0 {
            break;
        }
        if let Err(error) = run(line) {
            println!("Error: {}", error);
        };
    }
}
fn run(content: String) -> Result<(), Box<dyn Error>> {
    let mut scanner = Scanner::new(content);
    scanner.scan_tokens()?;
    scanner.display_tokens();
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let run_type = RunType::new(&args).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {}", error);
        process::exit(1);
    });
    match run_type {
        RunType::File(path) => run_file(&path),
        RunType::Prompt => run_prompt(),
    }
}
