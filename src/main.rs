pub mod token;
pub mod lexer;

use std::io::{self, BufRead, Write};
use token::Token;
use lexer::Lexer;

fn main() {
    let stdin = io::stdin();

    loop {
        print!(">> ");
        io::stdout().flush().expect("Error flushing stdout");

        let mut line = String::new();
        stdin.lock().read_line(&mut line).expect("Error reading from stdin");

        let mut lexer = Lexer::new(&mut line);

        loop {
            let token = lexer.next_token();
            println!("{:?}", token);

            if token == Token::EndOfFile {
                break;
            }
        }
    }
}
