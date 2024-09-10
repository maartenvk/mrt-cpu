#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Opcode {
    HLT,
    LDI,
    ADD
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
    R2
}

#[derive(Debug)]
pub enum RegisterConversionError {
    NoSuchRegister
}

impl TryFrom<&str> for Register {
    type Error = RegisterConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let result = match value.to_lowercase().as_str() {
            "r0" => Register::R0,
            "r1" => Register::R1,
            "r2" => Register::R2,
            _ => return Err(RegisterConversionError::NoSuchRegister)
        };

        Ok(result)
    }
}

impl TryFrom<u8> for Register {
    type Error = RegisterConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let result = match value {
            0 => Register::R0,
            1 => Register::R1,
            2 => Register::R2,
            _ => return Err(RegisterConversionError::NoSuchRegister)
        };

        Ok(result)
    }
}

#[derive(Debug)]
pub enum Instruction {
    NoParam(Opcode),
    RegImm(Opcode, Register, u8),
    TripleReg(Opcode, Register, Register, Register)
}
