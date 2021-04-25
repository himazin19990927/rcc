use inkwell::{builder::Builder, context::Context, module::Module, values::IntValue, AddressSpace};
#[allow(unused_imports)]
use rcc_parser::ast::{BinOp, Expr, Stmt, UnOp};
#[allow(unused_imports)]
use std::{cell::Cell, path::Path, rc::Rc};

pub struct Compiler<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(context: &'ctx Context) -> Compiler<'ctx> {
        let module = context.create_module("main");
        let builder = context.create_builder();

        let printf_type = {
            let i8_ptr_type = context.i8_type().ptr_type(AddressSpace::Generic);
            let i32_type = context.i32_type();

            i32_type.fn_type(&[i8_ptr_type.into()], true)
        };
        module.add_function("printf", printf_type, None);

        Compiler {
            context: context,
            module: module,
            builder: builder,
        }
    }
    pub fn build_program(&self, program: &Vec<Stmt>) {
        for stmt in program {
            self.build_stmt(stmt);
        }
    }

    pub fn build_stmt(&self, stmt: &Stmt) {
        match stmt {
            Stmt::Print(expr) => {
                let value = self.build_expr(expr);
                self.build_printf_int(value);
            }
            Stmt::Declaration(_, _) => unimplemented!("Stmt::Declaration"),
            Stmt::Assign(_, _) => unimplemented!("Stmt::Assign"),
            Stmt::Return(expr) => {
                let value = self.build_expr(expr);
                self.builder.build_return(Some(&value));
            }
        }
    }

    pub fn build_expr(&self, expr: &Expr) -> IntValue {
        match expr {
            Expr::Binary(op, left, right) => {
                let left_value = self.build_expr(left);
                let right_value = self.build_expr(right);

                match op {
                    BinOp::Add => self.builder.build_int_add(left_value, right_value, ""),
                    BinOp::Sub => self.builder.build_int_sub(left_value, right_value, ""),
                    BinOp::Mul => self.builder.build_int_mul(left_value, right_value, ""),
                    BinOp::Div => self
                        .builder
                        .build_int_unsigned_div(left_value, right_value, ""),
                    BinOp::Eq => unimplemented!(),
                    BinOp::Lt => unimplemented!(),
                    BinOp::Le => unimplemented!(),
                    BinOp::Ne => unimplemented!(),
                }
            }
            Expr::Unary(_, _) => unimplemented!(),
            Expr::Integer(value) => self.context.i32_type().const_int(*value, true),
            Expr::Ident(_) => unimplemented!(),
        }
    }

    pub fn build_main_func(&self) {
        let function = {
            let ty = self.context.i32_type().fn_type(&[], false);
            self.module.add_function("main", ty, None)
        };
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);
    }

    pub fn build_printf_int(&self, value: IntValue) {
        let printf_value = self
            .module
            .get_function("printf")
            .expect("cannot found function \"printf\"");

        let str_value = self
            .builder
            .build_global_string_ptr("%d\n", ".str")
            .as_pointer_value();

        self.builder
            .build_call(printf_value, &[str_value.into(), value.into()], "");
    }

    pub fn compile(&self, path: &Path) -> Result<(), String> {
        match self.module.print_to_file(path) {
            Ok(()) => Ok(()),
            Err(msg) => Err(msg.to_string()),
        }
    }
}
