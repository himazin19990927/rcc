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
            Expr::Binary(op, left, right) => {
                self.compile_expr(left);
                self.compile_expr(right);

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
                        unimplemented!()
                    }
                    _ => unimplemented!(),
                }

                self.builder.instr(Instr::Push(Reg::RAX));
            }
            Expr::Unary(op, right) => match op {
                UnOp::Neg => {
                    self.builder.instr(Instr::PushImm(0));
                    self.compile_expr(right);

                    self.builder.instr(Instr::Pop(Reg::RDI));
                    self.builder.instr(Instr::Pop(Reg::RAX));

                    self.builder.instr(Instr::Sub(Reg::RAX, Reg::RDI));

                    self.builder.instr(Instr::Push(Reg::RAX));
                }
            },
            Expr::Integer(num) => {
                self.builder.instr(Instr::PushImm(*num));
            }
        }
    }
}
