#[allow(dead_code)]
#[derive(Debug)]
pub struct Token {
    token_t: TokenT,
    lex: String,
    ln: usize
}

#[derive(Debug)]
pub enum TokenT {
    // Single-character tokens
    LEFTPAREN, RIGHTPAREN, LEFTBRACE, RIGHTBRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // One or two character tokens
    BANG, BANGEQUAL,
    EQUAL, EQUALEQUAL,
    GREATER, GREATEREQUAL,
    LESS, LESSEQUAL,

    // Literals
    IDENTIFIER(String), STRING(String), NUMBER(f64),

    // Keywords
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE, 

    EOF
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