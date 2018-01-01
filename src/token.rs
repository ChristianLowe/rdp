
#[derive(Debug, PartialEq)]
pub enum Token {
    // Special
    Illegal,
    EndOfFile,

    // Literals
    Ident(String),
    Integer(String),

    // Operators
    Assign,
    Declare,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LowerThan,
    GreaterThan,
    Equal,
    NotEqual,

    // Delimiters
    Comma,
    Colon,
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,

    // Keywords
    Function,
    True,
    False,
    If,
    Else,
    Return
}

impl Default for Token {
    fn default() -> Token {
        Token::Illegal
    }
}

pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        "def"   => Token::Function,
        "true"  => Token::True,
        "false" => Token::False,
        "if"    => Token::If,
        "else"  => Token::Else,
        "return"=> Token::Return,
        _       => Token::Ident(ident.to_string())
    }
}

#[test]
fn lookup_ident_test() {
    assert_eq!(lookup_ident("def"), Token::Function);

    let ident_token = lookup_ident("asdf");
    match ident_token {
        Token::Ident(str) => assert_eq!(str, "asdf"),
        _                 => assert!(false),
    }
}
