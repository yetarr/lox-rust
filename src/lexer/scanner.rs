use phf::phf_map;

use crate::{lox::Lox, utils};
use super::token::{Keyword, Token, TokenT, LitVal};

pub struct Scanner {
    src: String,
    tkns: Vec<Token>,
    start: usize,
    cur: usize,
    ln: usize
}

static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
    "and"    => Keyword::And,
    "class"  => Keyword::Class,
    "else"   => Keyword::Else,
    "false"  => Keyword::False,
    "for"    => Keyword::For,
    "fun"    => Keyword::Fun,
    "if"     => Keyword::If,
    "nil"    => Keyword::Nil,
    "or"     => Keyword::Or,
    "print"  => Keyword::Print,
    "return" => Keyword::Return,
    "super"  => Keyword::Super,
    "this"   => Keyword::This,
    "true"   => Keyword::True,
    "var"    => Keyword::Var,
    "while"  => Keyword::While,
};

fn lookup_keyword(ident: &str) -> Option<TokenT> {
    if let Some(k) = KEYWORDS.get(ident).copied() {
        return Some(TokenT::Keyword(k));
    }
    None
}

impl Scanner {
    pub fn new(src: String) -> Self {
        Scanner {
            src,
            tkns: Vec::new(),
            start: 0,
            cur: 0,
            ln: 1 ,
        }
    }

    pub fn scan_tokens(&mut self, mut lox: &mut Lox) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.cur;
            self.scan_token(&mut lox);
        }

        self.add_token(TokenT::EOF);
        &self.tkns
    }

    fn is_at_end(&self) -> bool {
        self.cur >= self.src.len()
    }

    fn scan_token(&mut self, lox: &mut Lox) {
        let c = self.advance();
        match c {
            ' ' | '\r' | '\t' => {}
            '\n' => self.ln += 1,
            '('  => self.add_token(TokenT::LeftParen),
            ')'  => self.add_token(TokenT::RightParen),
            '{'  => self.add_token(TokenT::LeftBrace),
            '}'  => self.add_token(TokenT::RightBrace),
            ','  => self.add_token(TokenT::Comma),
            '.'  => self.add_token(TokenT::Dot),
            '-'  => self.add_token(TokenT::Minus),
            '+'  => self.add_token(TokenT::Plus),
            ';'  => self.add_token(TokenT::Semicolon),
            '*'  => self.add_token(TokenT::Star),
            '!'  => {
                let token_t = if self.match_advance('=') { TokenT::BangEqual } else { TokenT::Bang };
                self.add_token(token_t);
            }
            '='  => {
                let token_t = if self.match_advance('=') { TokenT::EqualEqual } else { TokenT::Equal };
                self.add_token(token_t);
            }
            '<'  => {
                let token_t = if self.match_advance('=') { TokenT::LessEqual } else { TokenT::Less };
                self.add_token(token_t);
            }
            '>'  => {
                let token_t = if self.match_advance('=') { TokenT::GreaterEqual } else { TokenT::Greater };
                self.add_token(token_t);
            }
            '/'  => {
                if self.match_advance('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_advance('*') {
                    while !(self.peek() == '*' && self.peek_next() == '/') && !self.is_at_end() {
                        if self.peek() == '\n' { self.ln += 1; }
                        self.advance();
                    }

                    if !self.is_at_end() { self.advance(); }
                    else { 
                        lox.error(self.ln, "Unterminated block comment");
                        return;
                    }
                    
                    if !self.is_at_end() { self.advance(); }
                    else { lox.error(self.ln, "Unterminated block comment"); }
                } else {
                    self.add_token(TokenT::Slash);
                }
            }
            '"'  => self.string(lox),
            _    => {
                if utils::is_number(c) {
                    self.number();
                } else if utils::is_alpha(c) {
                    self.identifier();
                } else {
                    lox.error(self.ln, "Unexpected character");
                }
            },
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() { return '\0'; }
        self.src.as_bytes()[self.cur] as char
    }

    fn peek_next(&self) -> char {
        if self.cur + 1 >= self.src.len() { return '\0'; }
        self.src.as_bytes()[self.cur + 1] as char
    }

    fn advance(&mut self) -> char {
        self.cur += 1;
        self.cur_char()
    }

    fn match_advance(&mut self, exp: char) -> bool {
        if self.is_at_end() { return false; }
        if self.peek() != exp { return false; }

        self.cur += 1;
        true
    }

    fn add_token(&mut self, token_t: TokenT) {
        self.add_token_lit(token_t, LitVal::Nil);
    }

    fn add_token_lit(&mut self, token_t: TokenT, lit: LitVal) {
        let mut txt = "";
        if token_t != TokenT::EOF {
            txt = &self.src[self.start..self.cur];
        }
        self.tkns.push(Token::new(token_t, txt.to_string(), lit, self.ln));
    }

    fn cur_char(&self) -> char {
        self.src.as_bytes()[self.cur - 1] as char
    }

    fn string(&mut self, lox: &mut Lox) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.ln += 1 }
            self.advance();
        }

        if self.is_at_end() {
            lox.error(self.ln, "Unterminated string");
            return;
        }

        self.advance();

        let lit = &self.src[self.start + 1..self.cur - 1];
        self.add_token_lit(TokenT::Literal, LitVal::String(lit.to_string()));
    }

    fn number(&mut self) {
        while utils::is_number(self.peek()) { self.advance(); }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() { self.advance(); }
        }

        let lit = self.src[self.start..self.cur].parse::<f64>().unwrap();
        self.add_token_lit(TokenT::Literal, LitVal::Number(lit));
    }

    fn identifier(&mut self) {
        while utils::is_alpha_numeric(self.peek()) { self.advance(); }

        let id = &self.src[self.start..self.cur];
        match lookup_keyword(id) {
            Some(tt) => match tt {
                TokenT::Keyword(Keyword::True)  => self.add_token_lit(tt, LitVal::Boolean(true)),
                TokenT::Keyword(Keyword::False) => self.add_token_lit(tt, LitVal::Boolean(false)),
                _ => self.add_token(tt)
            },
            None => self.add_token(TokenT::Identifier),
        }
    }
}

