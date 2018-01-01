use token;
use token::Token;

use std::str::Chars;
use std::iter::Peekable;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer { input: input.chars().peekable() }
    }

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn peek_char_eq(&mut self, ch: char) -> bool {
        match self.peek_char() {
            Some(&peek_ch)  => peek_ch == ch,
            None            => false,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.peek_char() {
            if !ch.is_whitespace() {
                break;
            }

            self.read_char();
        }
    }

    fn peek_is_letter(&mut self) -> bool {
        match self.peek_char() {
            Some(&ch) => is_letter(ch),
            None      => false,
        }
    }

    fn peek_is_number(&mut self) -> bool {
        match self.peek_char() {
            Some(&ch) => ch.is_numeric(),
            None      => false,
        }
    }

    fn read_identifier(&mut self, first: char) -> String {
        let mut ident = String::new();
        ident.push(first);

        while self.peek_is_letter() {
            ident.push(self.read_char().unwrap());
        }

        ident
    }

    fn read_number(&mut self, first: char) -> String {
        let mut number = String::new();
        number.push(first);

        while self.peek_is_number() {
            number.push(self.read_char().unwrap());
        }

        number
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.read_char() {
            Some(':') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::Declare
                } else {
                    Token::Colon
                }
            }
            Some('=') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            Some('!') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }

            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('/') => Token::Slash,
            Some('*') => Token::Asterisk,
            Some('<') => Token::LowerThan,
            Some('>') => Token::GreaterThan,
            Some(',') => Token::Comma,
            Some('{') => Token::LeftBrace,
            Some('}') => Token::RightBrace,
            Some('(') => Token::LeftParenthesis,
            Some(')') => Token::RightParenthesis,

            Some(ch @ _) => {
                if is_letter(ch) {
                    let identifier = self.read_identifier(ch);
                    token::lookup_ident(&identifier)
                } else if ch.is_numeric() {
                    Token::Integer(self.read_number(ch))
                } else {
                    Token::Illegal
                }
            }

            None => Token::EndOfFile
        }
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

#[test]
fn next_token_test() {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let input = "
a := 5
b := 6

c := a + b

a = 11
c := a == c
opp := !c

if 6 > 5:
    return c
else:
    return false
";

    let tests = vec![
        Token::Ident("a".to_string()),
        Token::Declare,
        Token::Integer("5".to_string()),

        Token::Ident("b".to_string()),
        Token::Declare,
        Token::Integer("6".to_string()),

        Token::Ident("c".to_string()),
        Token::Declare,
        Token::Ident("a".to_string()),
        Token::Plus,
        Token::Ident("b".to_string()),

        Token::Ident("a".to_string()),
        Token::Assign,
        Token::Integer("11".to_string()),

        Token::Ident("c".to_string()),
        Token::Declare,
        Token::Ident("a".to_string()),
        Token::Equal,
        Token::Ident("c".to_string()),

        Token::Ident("opp".to_string()),
        Token::Declare,
        Token::Bang,
        Token::Ident("c".to_string()),

        Token::If,
        Token::Integer("6".to_string()),
        Token::GreaterThan,
        Token::Integer("5".to_string()),
        Token::Colon,

        Token::Return,
        Token::Ident("c".to_string()),

        Token::Else,
        Token::Colon,

        Token::Return,
        Token::False,

        Token::EndOfFile
    ];

    let mut lexer = Lexer::new(input);
    for test in tests {
        let token = lexer.next_token();
        assert_eq!(token, test);
    }
}
