#[allow(dead_code)]
#[derive(Debug)]
pub struct Token {
    token_t: TokenT,
    lex: String,
    ln: usize
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenT {
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals
    Keyword(Keyword), Identifier, String(String), Number(f64),

    EOF
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While, 
}

impl Token {
    pub fn new(token_t: TokenT, lex: String, ln: usize) -> Self {
        Token {
            token_t,
            lex,
            ln
        }
    }
    
    pub fn to_string(&self) -> String {
        format!("{:?} {}", self.token_t, self.lex)
    }
}