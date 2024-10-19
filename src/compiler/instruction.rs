use std::fmt::Display;

use crate::types::*;

use super::compiler::{CompileError, Token};

#[derive(Debug)]
pub enum Instruction {
    NoParam(Opcode),
    RegImm(Opcode, Register, u8),
    DoubleReg(Opcode, Register, Register),
    DoubleRegImm4(Opcode, Register, Register, u8),
    TripleReg(Opcode, Register, Register, Register),
}

impl Instruction {
    pub fn disassemble(first_byte: u8, second_byte: u8) -> Result<Instruction, CompileError> {
        let opcode_raw = first_byte >> 4;
        let opcode = Opcode::try_from(opcode_raw);
        if opcode.is_err() {
            println!("Error: Disassembly failed: Unknown opcode {}", opcode_raw);
            return Err(CompileError::UnexpectedEOF);
        }

        let opcode = opcode.unwrap();
        let reg_raw = first_byte & 0b1111;

        let get_reg = |raw: u8| {
            if let Ok(result) = Register::try_from(raw) {
                Ok(Token::Register(result))
            } else {
                Err(CompileError::UnexpectedEOF)
            }
        };

        let imm = || Token::Immediate(second_byte);

        let imm4 = || Token::Immediate(second_byte & 0b1111);

        let mut tokens = match Instruction::get_type(opcode) {
            InstructionType::NoParam => vec![],

            InstructionType::RegImm => vec![get_reg(reg_raw), Ok(imm())],

            InstructionType::DoubleReg => vec![get_reg(reg_raw), get_reg(second_byte >> 4)],

            InstructionType::DoubleRegImm4 => {
                vec![get_reg(reg_raw), get_reg(second_byte >> 4), Ok(imm4())]
            }

            InstructionType::TripleReg => vec![
                get_reg(reg_raw),
                get_reg(second_byte >> 4),
                get_reg(second_byte & 0b1111),
            ],
        }
        .into_iter();

        let generated = Instruction::generate(opcode, || {
            if let Some(token) = tokens.next() {
                token
            } else {
                Err(CompileError::UnexpectedEOF)
            }
        });

        Ok(generated?)
    }

    pub fn get_type(opcode: Opcode) -> InstructionType {
        match opcode {
            Opcode::HLT => InstructionType::NoParam,
            Opcode::LDI => InstructionType::RegImm,
            Opcode::ADD => InstructionType::TripleReg,
            Opcode::SB => InstructionType::TripleReg,
            Opcode::LB => InstructionType::TripleReg,
            Opcode::JNZ => InstructionType::DoubleReg,
            Opcode::JAL => InstructionType::TripleReg,
            Opcode::XOR => InstructionType::TripleReg,
            Opcode::SUB => InstructionType::TripleReg,
            Opcode::SHL => InstructionType::DoubleRegImm4,
            Opcode::SHR => InstructionType::DoubleRegImm4,
            Opcode::JC => InstructionType::DoubleReg,
            Opcode::NOT => InstructionType::DoubleReg,
            Opcode::AND => InstructionType::TripleReg,
            Opcode::OR => InstructionType::TripleReg,
        }
    }

    pub fn get_length(opcode: Opcode) -> u16 {
        match Self::get_type(opcode) {
            InstructionType::NoParam => 1,
            InstructionType::RegImm => 2,
            InstructionType::DoubleReg => 2,
            InstructionType::DoubleRegImm4 => 2,
            InstructionType::TripleReg => 2,
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoParam(opcode) => write!(f, "{:?}", opcode),

            Self::RegImm(opcode, reg, imm) => write!(f, "{:?} {:?} {:#02x}", opcode, reg, imm),

            Self::DoubleReg(opcode, reg, regb) => write!(f, "{:?} {:?} {:?}", opcode, reg, regb),

            Self::DoubleRegImm4(opcode, reg, regb, imm4) => {
                write!(f, "{:?} {:?} {:?} {:?}", opcode, reg, regb, imm4)
            }

            Self::TripleReg(opcode, reg, regb, regc) => {
                write!(f, "{:?} {:?} {:?} {:?}", opcode, reg, regb, regc)
            }
        }
    }
}

impl Instruction {
    pub fn serialize(&self) -> Vec<u8> {
        return match self {
            Self::NoParam(opcode) => vec![(*opcode as u8) << 4],
            Self::RegImm(opcode, reg, imm) => vec![(*opcode as u8) << 4 | *reg as u8, *imm],
            Self::DoubleReg(opcode, reg, reg2) => {
                vec![(*opcode as u8) << 4 | *reg as u8, (*reg2 as u8) << 4]
            }
            Self::DoubleRegImm4(opcode, reg, reg2, imm4) => {
                vec![(*opcode as u8) << 4 | *reg as u8, (*reg2 as u8) << 4 | imm4]
            }
            Self::TripleReg(opcode, reg, reg2, reg3) => vec![
                (*opcode as u8) << 4 | *reg as u8,
                (*reg2 as u8) << 4 | *reg3 as u8,
            ],
        };
    }

    pub fn generate<F>(opcode: Opcode, mut consumer: F) -> Result<Self, CompileError>
    where
        F: FnMut() -> Result<Token, CompileError>,
    {
        let reg = |consume_token: &mut F| match consume_token()? {
            Token::Register(reg) => Ok(reg),
            token => Err(CompileError::UnexpectedTokenType(token)),
        };

        let imm = |consume_token: &mut F| match consume_token()? {
            Token::Immediate(val) => Ok(val),
            token => Err(CompileError::UnexpectedTokenType(token)),
        };

        let create_no_param = |opcode| Instruction::NoParam(opcode);

        let create_reg_imm = |opcode, consumer: &mut F| -> Result<Instruction, CompileError> {
            Ok(Instruction::RegImm(opcode, reg(consumer)?, imm(consumer)?))
        };

        let create_double_reg = |opcode, consumer: &mut F| -> Result<Instruction, CompileError> {
            Ok(Instruction::DoubleReg(
                opcode,
                reg(consumer)?,
                reg(consumer)?,
            ))
        };

        let create_double_reg_imm4 =
            |opcode, consumer: &mut F| -> Result<Instruction, CompileError> {
                Ok(Instruction::DoubleRegImm4(
                    opcode,
                    reg(consumer)?,
                    reg(consumer)?,
                    imm(consumer)?,
                ))
            };

        let create_triple_reg = |opcode, consumer: &mut F| -> Result<Instruction, CompileError> {
            Ok(Instruction::TripleReg(
                opcode,
                reg(consumer)?,
                reg(consumer)?,
                reg(consumer)?,
            ))
        };

        let itype = Instruction::get_type(opcode);
        Ok(match itype {
            InstructionType::NoParam => create_no_param(opcode),
            InstructionType::RegImm => create_reg_imm(opcode, &mut consumer)?,
            InstructionType::DoubleReg => create_double_reg(opcode, &mut consumer)?,
            InstructionType::DoubleRegImm4 => create_double_reg_imm4(opcode, &mut consumer)?,
            InstructionType::TripleReg => create_triple_reg(opcode, &mut consumer)?,
        })
    }
}
