pub mod token;

use phf::phf_map;

use crate::{lox::Lox, utils};
use token::{Keyword, Token, TokenT, LitVal};

pub struct Scanner<'a> {
    src: String,
    tkns: Vec<Token>,
    start: usize,
    cur: usize,
    ln: usize,
    lox: &'a mut Lox
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

impl<'a> Scanner<'a> {
    pub fn new(src: String, lox: &'a mut Lox) -> Self {
        Scanner {
            src,
            tkns: Vec::new(),
            start: 0,
            cur: 0,
            ln: 1,
            lox,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.cur;
            self.scan_tkn();
        }

        self.add_tkn(TokenT::EOF);
        std::mem::take(&mut self.tkns)
    }

    fn is_at_end(&self) -> bool {
        self.cur >= self.src.len()
    }

    fn scan_tkn(&mut self) {
        let c = self.next();
        match c {
            ' ' | '\r' | '\t' => {}
            '\n' => self.ln += 1,
            '('  => self.add_tkn(TokenT::LeftParen),
            ')'  => self.add_tkn(TokenT::RightParen),
            '{'  => self.add_tkn(TokenT::LeftBrace),
            '}'  => self.add_tkn(TokenT::RightBrace),
            ','  => self.add_tkn(TokenT::Comma),
            '.'  => self.add_tkn(TokenT::Dot),
            '-'  => self.add_tkn(TokenT::Minus),
            '+'  => self.add_tkn(TokenT::Plus),
            ';'  => self.add_tkn(TokenT::Semicolon),
            ':'  => self.add_tkn(TokenT::Colon),
            '*'  => self.add_tkn(TokenT::Star),
            '?'  => self.add_tkn(TokenT::Query),
            '!'  => {
                let token_t = if self.match_nxt('=') { TokenT::BangEqual } else { TokenT::Bang };
                self.add_tkn(token_t);
            }
            '='  => {
                let token_t = if self.match_nxt('=') { TokenT::EqualEqual } else { TokenT::Equal };
                self.add_tkn(token_t);
            }
            '<'  => {
                let token_t = if self.match_nxt('=') { TokenT::LessEqual } else { TokenT::Less };
                self.add_tkn(token_t);
            }
            '>'  => {
                let token_t = if self.match_nxt('=') { TokenT::GreaterEqual } else { TokenT::Greater };
                self.add_tkn(token_t);
            }
            '/'  => {
                if self.match_nxt('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.next();
                    }
                } else if self.match_nxt('*') {
                    while !(self.peek() == '*' && self.peek_nxt() == '/') && !self.is_at_end() {
                        if self.peek() == '\n' { self.ln += 1; }
                        self.next();
                    }

                    if !self.is_at_end() { self.next(); }
                    else { 
                        self.lox.error_simple(self.ln, "Unterminated block comment");
                        return;
                    }
                    
                    if !self.is_at_end() { self.next(); }
                    else { self.lox.error_simple(self.ln, "Unterminated block comment"); }
                } else {
                    self.add_tkn(TokenT::Slash);
                }
            }
            '"'  => self.string(),
            _    => {
                if utils::is_number(c) {
                    self.number();
                } else if utils::is_alpha(c) {
                    self.identifier();
                } else {
                    self.lox.error_simple(self.ln, "Unexpected character");
                }
            },
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() { return '\0'; }
        self.src.as_bytes()[self.cur] as char
    }

    fn peek_nxt(&self) -> char {
        if self.cur + 1 >= self.src.len() { return '\0'; }
        self.src.as_bytes()[self.cur + 1] as char
    }

    fn next(&mut self) -> char {
        self.cur += 1;
        self.cur_char()
    }

    fn match_nxt(&mut self, exp: char) -> bool {
        if self.is_at_end() { return false; }
        if self.peek() != exp { return false; }

        self.cur += 1;
        true
    }

    fn add_tkn(&mut self, token_t: TokenT) {
        self.add_tkn_lit(token_t, LitVal::Nil);
    }

    fn add_tkn_lit(&mut self, token_t: TokenT, lit: LitVal) {
        let mut txt = "";
        if token_t != TokenT::EOF {
            txt = &self.src[self.start..self.cur];
        }
        self.tkns.push(Token::new(token_t, txt.to_string(), lit, self.ln));
    }

    fn cur_char(&self) -> char {
        self.src.as_bytes()[self.cur - 1] as char
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.ln += 1 }
            self.next();
        }

        if self.is_at_end() {
            self.lox.error_simple(self.ln, "Unterminated string");
            return;
        }

        self.next();

        let lit = &self.src[self.start + 1..self.cur - 1];
        self.add_tkn_lit(TokenT::Literal, LitVal::String(lit.to_string()));
    }

    fn number(&mut self) {
        while utils::is_number(self.peek()) { self.next(); }

        if self.peek() == '.' && self.peek_nxt().is_ascii_digit() {
            self.next();
            while self.peek().is_ascii_digit() { self.next(); }
        }

        let lit = self.src[self.start..self.cur].parse::<f64>().unwrap();
        self.add_tkn_lit(TokenT::Literal, LitVal::Number(lit));
    }

    fn identifier(&mut self) {
        while utils::is_alpha_numeric(self.peek()) { self.next(); }

        let id = &self.src[self.start..self.cur];
        match lookup_keyword(id) {
            Some(tt) => match tt {
                TokenT::Keyword(Keyword::True)  => self.add_tkn_lit(tt, LitVal::Boolean(true)),
                TokenT::Keyword(Keyword::False) => self.add_tkn_lit(tt, LitVal::Boolean(false)),
                _ => self.add_tkn(tt)
            },
            None => self.add_tkn(TokenT::Identifier),
        }
    }
}

