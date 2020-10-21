use scanner::Scanner;
use std::cmp::Ordering;
use std::env;
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
            Ordering::Less => {
                let path = args[1].clone();
                Ok(RunType::File(path))
            }
            Ordering::Equal => Ok(RunType::Prompt),
        }
    }
}

pub fn run_file(path: &str) {
    let file_contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    run(file_contents);
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
        run(line);
    }
}
fn run(content: String) {
    let mut scanner = Scanner::new(content);
    scanner.scan_tokens();
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
