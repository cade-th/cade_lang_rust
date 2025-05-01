use std::env;
use std::process;

pub mod cade_lang;
pub mod lexer;

use crate::cade_lang::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    cade_lang::run(&config);
}
