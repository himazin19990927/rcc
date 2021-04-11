use rcc_codegen::asm::{Builder, Instruction, Reg};
use std::fs;
use std::io::{BufWriter, Write};
use std::process::Command;

use clap::{App, Arg};

fn main() -> std::io::Result<()> {
    let matches = App::new("rcc")
        .arg(Arg::with_name("ASM_FILE_NAME").required(true).index(1))
        .arg(Arg::with_name("EXE_FILE_NAME").required(true).index(2))
        .get_matches();

    let asm_file_name = matches.value_of("ASM_FILE_NAME").unwrap();
    let exe_file_name = matches.value_of("EXE_FILE_NAME").unwrap();

    let mut builder = Builder::new();
    builder.label("main");
    builder.instr(Instruction::MovImm(Reg::RAX, 5));
    builder.instr(Instruction::AddImm(Reg::RAX, 20));
    builder.instr(Instruction::Ret);
    let asm = builder.build();

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
        .spawn()
        .expect("failed to start cc");
}
