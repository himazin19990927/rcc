use rcc_compiler::compiler::Compiler;
use rcc_parser::{lexer::Lexer, parser::Parser};
use std::fs;
use std::io::{BufWriter, Write};
use std::process::Command;

use clap::{App, Arg};

fn main() -> std::io::Result<()> {
    let matches = App::new("rcc")
        .arg(Arg::with_name("SOURCE_CODE").required(true).index(1))
        .get_matches();

    let asm_file_name = "tmp.s";
    let exe_file_name = "tmp";

    let src = matches.value_of("SOURCE_CODE").unwrap();
    let ast = Parser::new(Lexer::new(src)).expr();
    let asm = Compiler::new().compile(&ast);

    let mut f = BufWriter::new(fs::File::create(asm_file_name).unwrap());
    write!(f, "{}", asm)?;
    f.flush()?;

    run_assembler(exe_file_name, asm_file_name);

    Ok(())
}

fn run_assembler(exe_file: &str, asm_file: &str) {
    let _ = Command::new("cc")
        .arg("-o")
        .arg(exe_file)
        .arg(asm_file)
        .output()
        .expect("failed to start cc");
}
