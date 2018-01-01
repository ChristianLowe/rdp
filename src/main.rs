pub mod lexer;
pub mod token;

use token::Token;

#[derive(Debug)]
enum TokenType {
    Product,
    Sum,
    Number(i64),
    Paren,
}

#[derive(Debug)]
enum LexItem {
    Paren(char),
    Op(char),
    Num(i64),
}

#[derive(Debug)]
struct ParseNode {
    children: Vec<ParseNode>,
    entry: TokenType
}

impl ParseNode {
    pub fn new(token_type: TokenType) -> ParseNode {
        ParseNode {
            children: Vec::new(),
            entry: token_type,
        }
    }
}



fn main() {
    println!("Hello, world!");
}
