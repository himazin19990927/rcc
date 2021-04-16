use crate::asm::{Asm, Builder, Instr, Reg};
use rcc_parser::ast::{BinOp, Expr, UnOp};

pub struct Compiler {
    builder: Builder,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            builder: Builder::new(),
        }
    }

    pub fn compile(mut self, ast: &Expr) -> Asm {
        self.builder.label("main");
        self.compile_expr(ast);
        self.builder.instr(Instr::Pop(Reg::RAX));
        self.builder.instr(Instr::Ret);
        self.builder.build()
    }

    pub fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Binary(op, lhs, rhs) => self.compile_expr_binary(*op, lhs, rhs),
            Expr::Unary(op, rhs) => match op {
                UnOp::Neg => {
                    self.builder.instr(Instr::PushImm(0));
                    self.compile_expr(rhs);

                    self.builder.instr(Instr::Pop(Reg::RDI));
                    self.builder.instr(Instr::Pop(Reg::RAX));

                    self.builder.instr(Instr::Sub(Reg::RAX, Reg::RDI));

                    self.builder.instr(Instr::Push(Reg::RAX));
                }
            },
            Expr::Integer(num) => {
                self.builder.instr(Instr::PushImm(*num));
            }
            #[allow(unused_variables)]
            Expr::Ident(ident) => unimplemented!(),
        }
    }

    pub fn compile_expr_binary(&mut self, op: BinOp, lhs: &Expr, rhs: &Expr) {
        self.compile_expr(lhs);
        self.compile_expr(rhs);

        self.builder.instr(Instr::Pop(Reg::RDI));
        self.builder.instr(Instr::Pop(Reg::RAX));

        match op {
            BinOp::Add => {
                self.builder.instr(Instr::Add(Reg::RAX, Reg::RDI));
            }
            BinOp::Sub => {
                self.builder.instr(Instr::Sub(Reg::RAX, Reg::RDI));
            }
            BinOp::Mul => {
                self.builder.instr(Instr::Imul(Reg::RAX, Reg::RDI));
            }
            BinOp::Div => {
                self.builder.instr(Instr::Cqo);
                self.builder.instr(Instr::Idiv(Reg::RDI));
            }
            BinOp::Eq => {
                self.builder.instr(Instr::Cmp(Reg::RAX, Reg::RDI));
                self.builder.instr(Instr::Sete(Reg::Al));
                self.builder.instr(Instr::Movzb(Reg::RAX, Reg::Al));
            }
            BinOp::Lt => {
                self.builder.instr(Instr::Cmp(Reg::RAX, Reg::RDI));
                self.builder.instr(Instr::Setl(Reg::Al));
                self.builder.instr(Instr::Movzb(Reg::RAX, Reg::Al));
            }
            BinOp::Le => {
                self.builder.instr(Instr::Cmp(Reg::RAX, Reg::RDI));
                self.builder.instr(Instr::Setle(Reg::Al));
                self.builder.instr(Instr::Movzb(Reg::RAX, Reg::Al));
            }
            BinOp::Ne => {
                self.builder.instr(Instr::Cmp(Reg::RAX, Reg::RDI));
                self.builder.instr(Instr::Setne(Reg::Al));
                self.builder.instr(Instr::Movzb(Reg::RAX, Reg::Al));
            }
        }

        self.builder.instr(Instr::Push(Reg::RAX));
    }
}
