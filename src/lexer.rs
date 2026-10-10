use crate::token::{Token, TokenKind};

pub struct Lexer {
    src: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new (input: &str) -> Self {
        Self{
            src: input.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.src.get(self.pos).copied()
    }

    pub fn peek_next(&self) -> Option<char>{
        self.src.get(self.pos + 1).copied()
    }

    pub fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += 1;
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        }else{
            self.col += 1;
        }
        Some(ch)
    }

    fn lex_number(&mut self, first: char) -> TokenKind {
        let mut num_str = String::from(first);
        while let Some(ch) = self.peek(){
            if ch.is_ascii_digit(){
                num_str.push(ch);
                self.advance();
            }else{
                break;
            }
        }

        TokenKind::Int(num_str.parse().unwrap_or(0))
    }

    fn lex_ident(&mut self, first: char) -> TokenKind{
        let mut ident = String::from(first);
        while let Some(ch) = self.peek(){
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            }
            else{
                break;
            }
        }

        match ident.as_str(){
            "def" => TokenKind::Def,
            "return" => TokenKind::Return,
            "if" => TokenKind::If,
            "elif" => TokenKind::Elif,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "in" => TokenKind::In,
            "pass" => TokenKind::Pass,
            "True" => TokenKind::True,
            "False" => TokenKind::False,
            "None" => TokenKind::None,
            _=> TokenKind::Ident(ident),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let line = self.line;
        let col = self.col;

        let ch = match self.advance(){
            Some(c) => c,
            None => return Token::new(TokenKind::Eof, line, col),
        };

        let kind = match ch{
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            ':' => TokenKind::Colon,
            ',' => TokenKind::Comma,
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '=' => {
                if self.peek() == Some('='){
                    self.advance();
                    TokenKind::EqualEqual
                }else{
                    TokenKind::Equal
                }
            }

            '!' => {
                if self.peek() == Some('='){
                    self.advance();
                    TokenKind::NotEqual
                }else{
                    TokenKind::Ident("!".to_string())
                }
            }
            '<' => TokenKind::Less,
            '>' => TokenKind::Greater,
            c if c.is_ascii_digit() => self.lex_number(c),
            c if c.is_alphabetic() || c == '_' => self.lex_ident(c),

            _=> TokenKind::Ident(ch.to_string()),
        };

        Token::new(kind, line, col)
    }

    pub fn skip_whitespace(&mut self){
        while let Some(ch) = self.peek(){
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance();
            }else if ch == '#' {
                while let Some(c) = self.peek(){
                    self.advance();
                    if c == '\n' {
                        break;
                    }
                }
            }else{
                break;
            }
        }
    }

}


