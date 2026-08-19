#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Token {
    pub token_t: TokenT,
    pub lex: String,
    pub lit: LitVal,
    pub ln: usize
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
    Keyword(Keyword), Identifier, Literal,

    EOF
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While, 
}

#[derive(Debug, PartialEq, Clone)]
pub enum LitVal {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl LitVal {
    pub fn to_string(&self) -> String {
        match self {
            Self::String(s)  => s.clone(),
            Self::Number(n)  => n.to_string(),
            Self::Boolean(b) => b.to_string(),
            Self::Nil        => String::from("nil")
        }
    }
}

impl Token {
    pub fn new(token_t: TokenT, lex: String, lit: LitVal, ln: usize) -> Self {
        Token {
            token_t,
            lex,
            lit,
            ln
        }
    }
    
    pub fn to_string(&self) -> String {
        format!("{:?} {}", self.token_t, self.lex)
    }
}