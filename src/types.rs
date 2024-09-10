#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Opcode {
    HLT,
    LDI,
    ADD,
    SB,
    LB
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
    TripleReg
}

#[derive(Debug)]
pub enum Instruction {
    NoParam(Opcode),
    RegImm(Opcode, Register, u8),
    TripleReg(Opcode, Register, Register, Register)
}
