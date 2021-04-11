use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Reg {
    RAX,
    RDI,
    RSI,
    RDX,
    RCX,
    RBP,
    RSP,
    RBX,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Reg::RAX => write!(f, "rax"),
            Reg::RDI => write!(f, "rdi"),
            Reg::RSI => write!(f, "rsi"),
            Reg::RDX => write!(f, "rdx"),
            Reg::RCX => write!(f, "rcx"),
            Reg::RBP => write!(f, "rbp"),
            Reg::RSP => write!(f, "rsp"),
            Reg::RBX => write!(f, "rbx"),
            Reg::R8 => write!(f, "r8"),
            Reg::R9 => write!(f, "r9"),
            Reg::R10 => write!(f, "r10"),
            Reg::R11 => write!(f, "r11"),
            Reg::R12 => write!(f, "r12"),
            Reg::R13 => write!(f, "r13"),
            Reg::R14 => write!(f, "r14"),
            Reg::R15 => write!(f, "r15"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction {
    Mov(Reg, Reg),
    MovImm(Reg, u32),
    MovLd(Reg, Reg),
    MovSt(Reg, Reg),
    Push(Reg),
    PushImm(u32),
    Pop(Reg),

    Add(Reg, Reg),
    AddImm(Reg, u32),
    Sub(Reg, Reg),
    SubImm(Reg, u32),
    Imul(Reg, Reg),
    Idiv(Reg),

    Cqo,
    Ret,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Mov(dst, src) => write!(f, "mov {}, {}", dst, src),
            Instruction::MovImm(dst, imm) => write!(f, "mov {}, {}", dst, imm),
            Instruction::MovLd(dst, src) => write!(f, "mov {}, [{}]", dst, src),
            Instruction::MovSt(dst, src) => write!(f, "mov [{}], {}", dst, src),
            Instruction::Push(src) => write!(f, "push {}", src),
            Instruction::PushImm(imm) => write!(f, "push {}", imm),
            Instruction::Pop(dst) => write!(f, "pop {}", dst),

            Instruction::Add(dst, src) => write!(f, "add {}, {}", dst, src),
            Instruction::AddImm(dst, imm) => write!(f, "add {}, {}", dst, imm),
            Instruction::Sub(dst, src) => write!(f, "sub {}, {}", dst, src),
            Instruction::SubImm(dst, imm) => write!(f, "sub {}, {}", dst, imm),
            Instruction::Imul(dst, src) => write!(f, "imul {}, {}", dst, src),
            Instruction::Idiv(src) => write!(f, "idiv {}", src),

            Instruction::Cqo => write!(f, "cqo"),
            Instruction::Ret => write!(f, "ret"),
        }
    }
}

pub struct Asm {
    items: Vec<AsmItem>,
}

impl fmt::Display for Asm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, ".intel_syntax noprefix")?;
        writeln!(f, ".globl main")?;

        for item in &self.items {
            match &item {
                AsmItem::Label(label) => writeln!(f, "{}:", label)?,
                AsmItem::Instr(instr) => writeln!(f, "  {}", instr)?,
            }
        }

        Ok(())
    }
}

enum AsmItem {
    Label(String),
    Instr(Instruction),
}

pub struct Builder {
    asm: Asm,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            asm: Asm { items: Vec::new() },
        }
    }

    pub fn instr(&mut self, instr: Instruction) {
        self.asm.items.push(AsmItem::Instr(instr));
    }

    pub fn label<T: std::fmt::Display>(&mut self, label: T) {
        self.asm.items.push(AsmItem::Label(label.to_string()));
    }

    pub fn build(self) -> Asm {
        self.asm
    }
}
