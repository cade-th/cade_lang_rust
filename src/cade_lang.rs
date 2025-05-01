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

pub fn run(config: &Config) {
    let mut lexer: Lexer;
    if config.mode == Mode::SHELL {
        let mut should_quit = false;

        while should_quit {
            let input = run_shell();
            if input == ".quit" {
                should_quit = true;
            } else {
                lexer = Lexer::new(input);
                let tokens = lexer.lex();
            }
        }
    } else {
        let contents =
            fs::read_to_string(&config.file_path).expect("Should have been able to read the file");
        lexer = Lexer::new(contents);
        let tokens = lexer.lex();
    }
}

pub fn run_shell() -> String {
    print!(">>> ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim_end().to_string()
}
