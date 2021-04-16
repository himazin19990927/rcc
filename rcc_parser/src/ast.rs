#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum UnOp {
    Neg,
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Declaration(Expr, Expr),
    Assign(Expr, Expr),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Unary(UnOp, Box<Expr>),
    Integer(u32),
    Ident(String),
}
