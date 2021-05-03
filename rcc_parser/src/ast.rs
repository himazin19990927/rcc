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
    Ref,
    Deref,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TypeSpecifier {
    Void,
    Char,
    Int,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Declarator {
    Ident(String),
    Pointer(Box<Declarator>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Declaration {
    pub type_specifier: TypeSpecifier,
    pub declarator: Declarator,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    name: String,
    ret_type: Stmt,
    args: Vec<Declaration>,
    stmts: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Print(Expr),
    Declaration(Declaration),
    Assign(Expr, Expr),
    Return(Expr),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Unary(UnOp, Box<Expr>),
    Int(u64),
    Bool(bool),
    Ident(String),
}
