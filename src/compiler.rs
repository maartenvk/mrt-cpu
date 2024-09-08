use std::{fs::File, io::{Read, Write}};

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Opcode {
    HLT,
    LDI
}

impl From<&str> for Opcode {
    fn from(value: &str) -> Self {
        return match value.to_uppercase().as_str() {
            "HLT" => Opcode::HLT,
            "LDI" => Opcode::LDI,
            _ => panic!("No such opcode"),
        }
    }
}

#[repr(u8)]
pub enum Register {
    r0,
    r1,
    r2
}

pub enum Instruction {
    NoParam(Opcode),
    RegImm(Opcode, Register, u8),
    TripleReg(Opcode, Register, Register, Register)
}

impl Instruction {
    pub fn serialize(&self) -> Vec<u8> {
        return vec![*match self {
            Self::NoParam(opcode)
                => opcode,
            Self::RegImm(opcode, _, _)
                => opcode,
            Self::TripleReg(opcode, _, _, _)
                => opcode
        } as u8];
    }
}

pub struct Bytecode {
    instructions: Vec<Instruction>
}

impl Bytecode {
    pub fn new() -> Self {
        Self {
            instructions: vec![]
        }
    }

    pub fn create_binary(&self) -> Vec<u8> {
        let mut binary: Vec<u8> = vec![];
        for instruction in &self.instructions {
            binary.append(&mut instruction.serialize());
        }

        binary
    }
}

pub struct Compiler {
    input_file: File,
    output_file: File,
    generated: Bytecode
}

#[derive(Debug)]
pub enum CompileError {
    ReadFromInputFailed,
    WriteToOutputFailed
}

impl Compiler {
    pub fn new(input_file: File, output_file: File) -> Self {
        Self {
            input_file,
            output_file,
            generated: Bytecode::new()
        }
    }

    pub fn compile(&mut self) -> Result<(), CompileError> {
        let mut lines = String::new();
        if self.input_file.read_to_string(&mut lines).is_err() {
            return Err(CompileError::ReadFromInputFailed);
        }

        let mut line_number = 0;
        let chars = lines.chars();


        let lines = lines.lines().map(String::from).collect::<Vec<String>>();
        for line in lines {

        }

        // flush to output
        let binary = &self.generated.create_binary();
        if self.output_file.write_all(binary).is_err() {
            return Err(CompileError::WriteToOutputFailed);
        }

        Ok(())
    }
}