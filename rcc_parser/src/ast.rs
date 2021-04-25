#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,

    Eq,
    Lt,
    Le,
    Ne,
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
    Integer(u64),
    Ident(String),
}
