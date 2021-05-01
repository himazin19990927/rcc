use core::panic;
use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    values::{IntValue, PointerValue},
    AddressSpace,
};
#[allow(unused_imports)]
use rcc_parser::ast::{BinOp, Expr, Stmt, UnOp};
use std::collections::HashMap;
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
        let mut variables = HashMap::new();

        for stmt in program {
            self.build_stmt(stmt, &mut variables);
        }
    }

    pub fn build_stmt(&self, stmt: &Stmt, variables: &mut HashMap<String, PointerValue<'ctx>>) {
        match stmt {
            Stmt::Print(expr) => {
                let value = self.build_expr(expr, variables);
                self.build_printf_int(value);
            }
            Stmt::Declaration(lhs, rhs) => {
                if let Expr::Ident(ident) = lhs {
                    self.bulid_declaration(ident.clone(), rhs, variables);
                }
            }
            Stmt::Assign(_, _) => unimplemented!("Stmt::Assign"),
            Stmt::Return(expr) => {
                let value = self.build_expr(expr, variables);
                self.builder.build_return(Some(&value));
            }
        }
    }

    pub fn bulid_declaration(
        &self,
        name: String,
        rhs: &Expr,
        variables: &mut HashMap<String, PointerValue<'ctx>>,
    ) {
        let mem_ptr = self.builder.build_alloca(self.context.i32_type(), "local");
        let rhs_value = self.build_expr(rhs, variables);
        self.builder.build_store(mem_ptr, rhs_value);

        variables.insert(name, mem_ptr);
    }

    pub fn build_expr(
        &self,
        expr: &Expr,
        variables: &HashMap<String, PointerValue<'ctx>>,
    ) -> IntValue {
        match expr {
            Expr::Binary(op, left, right) => {
                let left_value = self.build_expr(left, variables);
                let right_value = self.build_expr(right, variables);

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
                    BinOp::And => unimplemented!(),
                    BinOp::Or => unimplemented!(),
                }
            }
            Expr::Unary(_, _) => unimplemented!(),
            Expr::Integer(value) => self.context.i32_type().const_int(*value, true),
            Expr::Ident(ident) => match variables.get(ident) {
                Some(ptr_value) => self
                    .builder
                    .build_load(ptr_value.clone(), "")
                    .into_int_value(),
                None => panic!("Cannot find value '{}' in this scope.", ident),
            },
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
