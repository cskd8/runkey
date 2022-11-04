mod token;
mod lexer;
mod repl;
use std::io;

fn main() {
    let user = whoami::username();
    println!("Hello, {}! This is the Runkey programming language!", user);
    println!("Feel free to type in commands");
    repl::repl::start(io::stdin(), io::stdout()).unwrap();
}
