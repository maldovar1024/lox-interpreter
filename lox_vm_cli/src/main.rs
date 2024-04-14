use std::{env, fs};

use lox_bytecode::compiler::Compiler;
use lox_resolver::Resolver;

fn run_from_file(file_path: &str) {
    let content =
        fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Cannot read file `{file_path}`"));
    match lox_parser::parse(&content) {
        Ok(mut ast) => match Resolver::default().resolve(&mut ast) {
            Some(errors) => errors.iter().for_each(|e| eprintln!("{e}")),
            None => {
                let mut compiler = Compiler::default();
                compiler.compile(&ast);
                println!("{:?}", compiler);
            }
        },
        Err(errors) => errors.iter().for_each(|e| eprintln!("{e}")),
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();

    run_from_file(&args[1]);
}
