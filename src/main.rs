mod error;
mod scan;
mod token_type;
mod token;

use error::Report;
use scan::Scanner;
use token::Token;
use std::{env, process, fs, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_args = args.len();
    dbg!(&args);

    match num_args {
        1 => run_prompt(),
        2 => run_file(&args[2]),
        _ => {
            println!("wrong number of arguments ({num_args})");
            println!("Usage: jlox [script]");
            process::exit(64);
        }
    }
}

fn run_file(path: &str) {
    let contents = fs::read_to_string(path)
        .expect(&format!("Should have been able to read the file. {path}"));

    let result = run(&contents);
    if let Err(report) = result {
        error::handle_error(report);
        process::exit(65);
    }
}

fn run_prompt() {
    let mut line = String::new();
    loop {
        print!("> ");
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if line.is_empty() {
            break;
        }
        let result = run(&line);
        if let Err(report) = result {
            error::handle_error(report);
        };
        line.clear();
    }
}

fn run(source: &str) -> Result<(), Report> {
    let mut scanner: Scanner = Scanner::new(source);
    let tokens= scanner.scan_tokens();

    // For now, just print the tokens.
    for token in tokens {
        println!("{token}");
    }
    Ok(())
}

fn print_error() {
    println!("Error");
}

// private static void run(String source) {
// Scanner scanner = new Scanner(source);
// List<Token> tokens = scanner.scanTokens();
//
// // For now, just print the tokens.
// for (Token token : tokens) {
// System.out.println(token);
// }
// }