use std::fmt::Display;

use crate::compiler::{CompileError, Token};

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Opcode {
    HLT,
    LDI,
    ADD,
    SB,
    LB,
    JNZ,
    JAL,
    XOR
}

#[derive(Debug)]
pub enum OpcodeConversionError {
    NoSuchOpcode
}

impl TryFrom<&str> for Opcode {
    type Error = OpcodeConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let result = match value.to_uppercase().as_str() {
            "HLT" => Opcode::HLT,
            "LDI" => Opcode::LDI,
            "ADD" => Opcode::ADD,
            "SB" => Opcode::SB,
            "LB" => Opcode::LB,
            "JNZ" => Opcode::JNZ,
            "JAL" => Opcode::JAL,
            "XOR" => Opcode::XOR,
            _ => return Err(OpcodeConversionError::NoSuchOpcode)
        };

        Ok(result)
    }
}

impl TryFrom<u8> for Opcode {
    type Error = OpcodeConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let result = match value {
            0 => Opcode::HLT,
            1 => Opcode::LDI,
            2 => Opcode::ADD,
            3 => Opcode::SB,
            4 => Opcode::LB,
            5 => Opcode::JNZ,
            6 => Opcode::JAL,
            7 => Opcode::XOR,
            _ => return Err(OpcodeConversionError::NoSuchOpcode)
        };

        Ok(result)
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R12,
    R13,
    R14,
    R15
}

#[derive(Debug)]
pub enum RegisterConversionError {
    NoSuchRegister
}

impl TryFrom<&str> for Register {
    type Error = RegisterConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let register = value.to_ascii_lowercase();
        let length = register.len();

        let fail = || Err(RegisterConversionError::NoSuchRegister);

        if length < 2 || length > 3 {
            return fail();
        }

        let mut chars = register.chars();
        if chars.next() != Some('r') {
            return fail();
        }

        let digit = chars.next();

        let mut num = (digit.unwrap() as u8) - ('0' as u8);
        
        if let Some(digit_2) = chars.next() {
            num *= 10;
            num += (digit_2 as u8) - ('0' as u8);
        }

        Register::try_from(num)
    }
}

impl TryFrom<u8> for Register {
    type Error = RegisterConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 15 {
            return Err(RegisterConversionError::NoSuchRegister) 
        }

        let regs = [
           Register::R0,
           Register::R1,
           Register::R2,
           Register::R3,
           Register::R4,
           Register::R5,
           Register::R6,
           Register::R7,
           Register::R8,
           Register::R9,
           Register::R10,
           Register::R12,
           Register::R13,
           Register::R14,
           Register::R15
        ];

        Ok(regs[value as usize])
    }
}

#[derive(Debug)]
pub enum InstructionType {
    NoParam,
    RegImm,
    DoubleReg,
    TripleReg
}

#[derive(Debug)]
pub enum Instruction {
    NoParam(Opcode),
    RegImm(Opcode, Register, u8),
    DoubleReg(Opcode, Register, Register),
    TripleReg(Opcode, Register, Register, Register)
}

impl Instruction {
    pub fn disassemble(first_byte: u8, second_byte: u8) -> Result<Instruction, CompileError>{
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

        let imm = || {
            Token::Immediate(second_byte)
        };

        let mut tokens = match Instruction::get_type(opcode) {
            InstructionType::NoParam
             => vec![],

            InstructionType::RegImm 
             => vec![get_reg(reg_raw), Ok(imm())],

            InstructionType::DoubleReg
             => vec![get_reg(reg_raw), get_reg(second_byte >> 4)],

            InstructionType::TripleReg
             => vec![get_reg(reg_raw), get_reg(second_byte >> 4), get_reg(second_byte & 0b1111)]
        }.into_iter();

        let generated = Instruction::generate(opcode, || {
            if let Some(token) = tokens.next() {
                token
            } else {
                Err(CompileError::UnexpectedEOF)
            }
        });

        Ok(generated?)
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoParam(opcode)
                => write!(f, "{:?}", opcode),

            Self::RegImm(opcode, reg, imm)
                => write!(f, "{:?} {:?} {:#02x}", opcode, reg, imm),

            Self::DoubleReg(opcode, reg, regb)
                => write!(f, "{:?} {:?} {:?}", opcode, reg, regb),

            Self::TripleReg(opcode, reg, regb, regc)
                => write!(f, "{:?} {:?} {:?} {:?}", opcode, reg, regb, regc)
        }
    }
}
