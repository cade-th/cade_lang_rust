use crate::lexer::Lexer;
use std::fs;
use std::io::{self, Write};

// Need this in order to have the equals in if statements
#[derive(PartialEq)]
pub enum Mode {
    SHELL,
    FILE,
}

pub struct Config {
    pub file_path: String,
    pub mode: Mode,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() == 1 {
            let file_path = String::from("none");
            let mode = Mode::SHELL;
            Ok(Config { file_path, mode })
        } else if args.len() == 2 {
            let file_path = args[1].clone();
            let mode = Mode::FILE;
            Ok(Config { file_path, mode })
        } else {
            Err("Enter a file or use the shell")
        }
    }
}

// Maybe do this here:
// pub fn run(config: &Config) -> Result<(), LexError> {
/*
and then this in main:
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|e| {
        eprintln!("Config error: {}", e);
        process::exit(1);
    });

    if let Err(e) = run(&config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
*/

pub fn run(config: &Config) {
    let mut lexer: Lexer;
    if config.mode == Mode::SHELL {
        loop {
            let input = run_shell();
            if input == ".quit" {
                break;
            } else {
                lexer = Lexer::new(input);
                match lexer.lex() {
                    Ok(tokens) => {
                        println!("Lexing succeeded. Tokens:");
                        for token in tokens {
                            println!("{:?}", token);
                        }
                    }
                    Err(e) => println!("Lexer error: {}", e),
                }
            }
        }
    } else {
        let contents =
            fs::read_to_string(&config.file_path).expect("Should have been able to read the file");
        lexer = Lexer::new(contents);
        match lexer.lex() {
            Ok(tokens) => {
                println!("Lexing succeeded. Tokens:");
                for token in tokens {
                    println!("{:?}", token);
                }
            }
            Err(e) => println!("Lexer error: {}", e),
        }
    }
}

pub fn run_shell() -> String {
    print!("cade_lang> ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim_end().to_string()
}
