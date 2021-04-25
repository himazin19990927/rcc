use inkwell::context::Context;
use rcc_compiler::compiler::Compiler;
use rcc_parser::{lexer::Lexer, parser::Parser};
use std::path::Path;

use clap::{App, Arg};

fn main() -> Result<(), String> {
    let matches = App::new("rcc")
        .arg(Arg::with_name("SOURCE_CODE").required(true).index(1))
        .get_matches();

    let ir_file_name = "tmp.ll";

    let src = matches.value_of("SOURCE_CODE").unwrap();
    let ast = Parser::new(Lexer::new(src)).program();

    let ctx = Context::create();

    let compiler = Compiler::new(&ctx);

    compiler.build_main_func();
    compiler.build_program(&ast);

    compiler.compile(Path::new(ir_file_name))?;

    Ok(())
}
