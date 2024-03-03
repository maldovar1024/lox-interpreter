use lox_parser;
use std::io::{self, Write};

fn main() {
    loop {
        print!(">");
        io::stdout().flush().unwrap();
        let mut content = String::new();
        io::stdin().read_line(&mut content).unwrap();

        if content.trim() == "@q" {
            return;
        }

        match lox_parser::parse(&content) {
            Some(ast) => println!("{ast:?}"),
            None => println!("ERROR!"),
        }
    }
}
