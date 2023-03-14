mod token;
mod lexer;
mod repl;

use std::env;
use std::io::{self};
use std::process;

fn main() {
    let user = match env::var("USER") {
        Ok(user) => user,
        _ => {
            eprintln!("Could not determine username.");
            process::exit(1);
        }
    };

    println!("Hello {}! This is the Monkey programming language!", user);
    println!("Feel free to type in commands");

    let stdin = io::stdin();
    let stdout = io::stdout();

    repl::start(&mut stdin.lock(), &mut stdout.lock());
}
