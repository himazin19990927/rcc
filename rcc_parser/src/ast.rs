#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinOp {
    Mul,
    Div,

    Add,
    Sub,

    Lt,
    Le,

    Eq,
    Ne,

    And,
    Or,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnOp {
    Neg,
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Print(Expr),
    Declaration(Expr, Expr),
    Assign(Expr, Expr),
    Return(Expr),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Unary(UnOp, Box<Expr>),
    Int(u64),
    Bool(bool),
    Ident(String),
}
