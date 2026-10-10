use crate::token::TokenKind;

#[derive(Debug, Clone, PartialEq)]

pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr{
    Literal(Literal),
    Ident(String),
    Binary{
        op: TokenKind,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Call{
        callee: String,
        args: Vec<Expr>,
    },
}


#[derive(Debug, Clone, PartialEq)]
pub enum Stmt{
    Expr(Expr),
    Assign{
        name: String,
        value: Expr,
    },

    Return(Option<Expr>),
    Pass,
    If{
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },

    While{
        condition: Expr,
        body: Vec<Stmt>,
    },
    FnDef{
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
}


#[derive(Debug, Clone, PartialEq)]
pub struct Program{
    pub stmts: Vec<Stmt>,
}




