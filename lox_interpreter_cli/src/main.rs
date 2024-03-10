use lox_interpreter::interpret;
use lox_parser;
use std::{
    env, fs,
    io::{self, Write},
};

fn run(src: &str) {
    match lox_parser::parse(&src) {
        Ok(ast) => {
            println!("{ast:?}");
            println!("{:?}", interpret(&ast));
        }
        Err(e) => eprintln!("{e}"),
    }
}

fn run_interactively() {
    loop {
        print!(">");
        io::stdout().flush().unwrap();
        let mut content = String::new();
        io::stdin().read_line(&mut content).unwrap();

        if content.trim() == "@q" {
            return;
        }

        run(&content);
    }
}

fn run_from_file(file_path: &str) {
    let content = fs::read_to_string(file_path).expect(&format!("Cannot read file `{file_path}`"));
    run(&content);
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        run_interactively();
    } else {
        run_from_file(&args[1]);
    }
}
