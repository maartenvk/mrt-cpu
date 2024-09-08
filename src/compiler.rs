use std::{fs::File, io::{Read, Write}};

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Opcode {
    HLT
}

impl From<&str> for Opcode {
    fn from(value: &str) -> Self {
        return match value.to_uppercase().as_str() {
            "HLT" => {
                Opcode::HLT
            },
            _ => {
                panic!("No such opcode");
            }
        }
    }
}

pub struct Instruction {
    opcode: Opcode
}

impl Instruction {
    pub fn serialize(&self) -> Vec<u8> {
        vec![self.opcode as u8]
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

impl Compiler {
    pub fn new(input_file: File, output_file: File) -> Self {
        Self {
            input_file,
            output_file,
            generated: Bytecode::new()
        }
    }

    pub fn compile(&mut self) {
        let mut lines = String::new();
        if self.input_file.read_to_string(&mut lines).is_err() {
            println!("Error: Failed to read from input");
            return;
        }

        let lines = lines.lines().map(String::from).collect::<Vec<String>>();

        for line in lines {
            let split = line.split(' ').collect::<Vec<_>>();
            self.generated.instructions.push(Instruction {
                opcode: Opcode::from(split[0])
            });
        }

        // flush to output
        let binary = &self.generated.create_binary();
        if self.output_file.write_all(binary).is_err() {
            println!("Error: Failed to write to output file");
            return;
        }

        println!("Info: Compilation succesful");
    }
}
