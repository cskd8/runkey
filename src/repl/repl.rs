use std::io::{self, Write};
use crate::lexer::*;
use crate::token::*;

pub const PROMPT: &str = ">> ";

pub fn start(r#in: io::Stdin, mut out: io::Stdout) -> io::Result<()> {
    let mut input = String::new();
    loop {
        out.write(PROMPT.as_bytes())?;
        out.flush()?;
        r#in.read_line(&mut input)?;
        let mut l = lexer::Lexer::new(input.clone());
        loop {
            let tok = l.next_token();
            println!("{:?}", tok);
            if tok.r#type == token::EOF.to_string() {
                break;
            }
        }
    }
}
