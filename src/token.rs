#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind{
    
    Def, 
    Return,
    If,
    Elif,
    Else,
    While,
    For,
    In,
    Pass,
    True,
    False,
    None,

    Ident(String),
    Int(i64),
    Float(f64),
    String(String),

    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    Greater,

    Colon,
    Comma,
    OpenParen,
    CloseParen,

    Newline,
    Indent,
    Dedent,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,

}

impl Token{
    pub fn new (kind: TokenKind, line: usize, col: usize) -> Self {
        Self {
            kind, 
            span: Span {line, col},
        }
    }
}




