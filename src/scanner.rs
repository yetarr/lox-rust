use crate::{lox::Lox, token::{Token, TokenT}};

#[allow(dead_code)]
pub struct Scanner {
    src: String,
    tkns: Vec<Token>,
    start: usize,
    cur: usize,
    ln: usize
}

impl Scanner {
    pub fn new(src: String) -> Self {
        Scanner { 
            src, 
            tkns: Vec::new(),
            start: 0,
            cur: 0,
            ln: 0,
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
            '('  => self.add_token(TokenT::LEFTPAREN),
            ')'  => self.add_token(TokenT::RIGHTPAREN),
            '{'  => self.add_token(TokenT::LEFTBRACE),
            '}'  => self.add_token(TokenT::RIGHTBRACE),
            ','  => self.add_token(TokenT::COMMA),
            '.'  => self.add_token(TokenT::DOT),
            '-'  => self.add_token(TokenT::MINUS),
            '+'  => self.add_token(TokenT::PLUS),
            ';'  => self.add_token(TokenT::SEMICOLON),
            '*'  => self.add_token(TokenT::STAR),
            '!'  => {
                let token_t = if self.match_advance('=') { TokenT::BANGEQUAL } else { TokenT::BANG };
                self.add_token(token_t);
            }
            '='  => {
                let token_t = if self.match_advance('=') { TokenT::EQUALEQUAL } else { TokenT::EQUAL };
                self.add_token(token_t);
            }
            '<'  => {
                let token_t = if self.match_advance('=') { TokenT::LESSEQUAL } else { TokenT::LESS };
                self.add_token(token_t);
            }
            '>'  => {
                let token_t = if self.match_advance('=') { TokenT::GREATEREQUAL } else { TokenT::GREATER };
                self.add_token(token_t);
            }
            '/'  => {
                if self.match_advance('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    } 
                } else {
                    self.add_token(TokenT::SLASH);
                }
            }
            '"'  => self.string(lox),
            _    => {
                if c.is_ascii_digit() {
                    self.number();
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
        self.add_token_lit(token_t);
    }

    fn add_token_lit(&mut self, token_t: TokenT) {
        let txt: &str = &self.src[self.start..self.cur + 1];
        self.tkns.push(
            Token::new(token_t, txt.to_string(), self.ln)
        );
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

        let lit = &self.src[self.start + 1..self.cur];
        self.add_token_lit(TokenT::STRING(lit.to_string()));
    } 

    fn number(&mut self) {
        while self.peek().is_ascii_digit() { self.advance(); }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() { self.advance(); }
        }

        let lit = &self.src[self.start + 1..self.cur].parse::<f64>().unwrap(); 
        self.add_token_lit(TokenT::NUMBER(*lit));
    }
}