use std::fmt::{Display, Write};

use crate::types::Opcode;

pub struct System {
    rom: Vec<u8>,
    ram: Vec<u8>,
    regs: [u8;16],
    ip: u16,
    flags: FlagsRegister
}

#[derive(Debug)]
pub enum LoadRomError {
    EmptyRom()
}

#[derive(Debug)]
pub enum LoadRamError {
    EmptyRam()
}

impl System {
    pub fn new(ram_size: usize) -> Self {
        Self {
            rom: vec![0u8; 1],
            ram: vec![0u8; ram_size],
            regs: [0;16],
            ip: 0,
            flags: FlagsRegister::new()
        }
    }

    pub fn get_mem(&self, address: u16) -> u8 {
        *self.ram.get(address as usize).unwrap_or_else(|| {
            println!("Error: out of bounds memory access [{:#06x}] ip={}", address, self.ip);
            return &0;
        })
    }

    pub fn set_mem(&mut self, address: u16, value: u8) {
        if let Some(reference) = self.ram.get_mut(address as usize) {
            *reference = value;
        } else {
            println!("Error: out of bounds memory access [{:#06x}] ip={}", address, self.ip);
        }
    }

    pub fn get_rom(&self, address: u16) -> u8 {
        *self.rom.get(address as usize).unwrap_or_else(|| {
            println!("Error: out of bounds ROM access [{:#06x}] ip={}", address, self.ip);
            return &0;
        })
    }

    pub fn get_regs(&self) -> [u8;16] {
        self.regs
    }

    pub fn get_ip(&self) -> u16 {
        self.ip
    }

    pub fn get_flags_register(&self) -> &FlagsRegister {
        &self.flags
    }

    pub fn jump(&mut self, address: u8) {
        self.ip = address as u16;
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) -> Result<(),LoadRomError> {
        if rom.is_empty() {
            return Err(LoadRomError::EmptyRom())
        }
        
        self.rom = rom;
        Ok(())
    }

    pub fn load_ram(&mut self, ram: Vec<u8>) -> Result<(),LoadRamError> {
        if ram.is_empty() {
            return Err(LoadRamError::EmptyRam())
        }

        self.ram = ram;
        Ok(())
    }

    // returns true if halted
    pub fn tick(&mut self) -> bool {
        let first_byte = *self.rom.get(self.ip as usize).unwrap_or(&0);
        let data = *self.rom.get(self.ip as usize + 1).unwrap_or(&0);

        let opcode_raw = first_byte >> 4;
        let opcode = Opcode::try_from(opcode_raw);
        if opcode.is_err() {
            println!("Info: Illegal Instruction: {}", opcode_raw);
            return false;
        }

        let opcode = opcode.unwrap();
        let reg_raw = (first_byte & 0b1111) as usize;
        let reg2_raw = (data >> 4) as usize;
        let reg3_raw = (data & 0b1111) as usize;

        let reg = self.regs.get(reg_raw);
        let reg2 = self.regs.get(reg2_raw);
        let reg3 = self.regs.get(reg3_raw);

        let imm = data;
        let offset = (*reg2.unwrap_or(&0) as usize) << 8 | *reg3.unwrap_or(&0) as usize;

        match opcode {
            Opcode::HLT => {
                println!("Info: halting at ip={}", self.ip);
                return true;
            },
            Opcode::LDI => {
                self.regs[reg_raw] = imm;
                self.ip += 2;
            },
            Opcode::ADD => {
                let alu = ALU::add(*reg2.unwrap(), *reg3.unwrap());
                self.flags = alu.flags;

                self.regs[reg_raw] = alu.result;
                self.ip += 2;
            },
            Opcode::SB => {
                self.set_mem(offset as u16, *reg.unwrap());
                self.ip += 2;
            },
            Opcode::LB => {
                self.regs[reg_raw] = self.get_mem(offset as u16);
                self.ip += 2;
            },
            Opcode::JNZ => {
                let zf_set = self.flags.is_set(Flags::Zero);
                if !zf_set {
                    self.ip = ((*reg.unwrap() as u16) << 8) | *reg2.unwrap() as u16;
                } else {
                    self.ip += 2;
                }
            },
            Opcode::JAL => {
                let new_ip = offset as u16;

                self.ip += 2; // Account for current instruction

                self.regs[reg_raw] = (self.ip >> 8) as u8;
                self.regs[reg2_raw] = self.ip as u8;

                self.ip = new_ip;
            },
            Opcode::XOR => {
                let alu = ALU::xor(*reg2.unwrap(), *reg3.unwrap());
                self.flags = alu.flags;

                self.regs[reg_raw] = alu.result;
                self.ip += 2;
            },
            Opcode::SUB => {
                let alu = ALU::sub(*reg2.unwrap(), *reg3.unwrap());
                self.flags = alu.flags;

                self.regs[reg_raw] = alu.result;
                self.ip += 2;
            },
        };

        false
    }
}

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum Flags {
    Zero,
    Carry,
    Sign,
    Overflow
}

impl Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Flags::Zero     => 'Z',
            Flags::Carry    => 'C',
            Flags::Sign     => 'S',
            Flags::Overflow => 'O'
        })
    }
}

pub struct FlagsRegister {
    flags: [bool; 4]
}

impl FlagsRegister {
    pub fn new() -> Self {
        Self {
            flags: [false; 4]
        }
    }

    pub fn with_flags(flags: Vec<Flags>) -> Self {
        let mut result = [false; 4];
        for flag in flags {
            result[flag as usize] = true;
        }
        
        Self {
            flags: result
        }
    }

    pub fn is_set(&self, flag: Flags) -> bool {
        self.flags[flag as usize]
    }

    pub fn get_flags(&self) -> Vec<Flags> {
        let mut result = vec![];

        let flags = [Flags::Zero, Flags::Carry, Flags::Sign, Flags::Overflow];
        for flag in flags {
            if self.is_set(flag.clone()) {
                result.push(flag);
            }
        }

        result
    }
}

pub struct ALU {
    result: u8,
    flags: FlagsRegister
}

impl ALU {
    fn is_signed(byte: u8) -> bool {
        (byte & 0b1000_0000) > 0
    }

    fn flags_for_operation(a: u8, b: u8, result: (u8, bool)) -> FlagsRegister {
        let mut flags = vec![];
        if result.0 == 0 {
            flags.push(Flags::Zero);
        }

        if result.1 {
            flags.push(Flags::Carry);
        }

        if ALU::is_signed(result.0) {
            flags.push(Flags::Sign);
        }

        // if both a, b are either signed or unsigned and different with result
        if ALU::is_signed(a) == ALU::is_signed(b) &&
            ALU::is_signed(a) != ALU::is_signed(result.0)
        {
            flags.push(Flags::Overflow);
        }

        FlagsRegister::with_flags(flags)
    }

    pub fn add(a: u8, b: u8) -> Self {
        let result = a.overflowing_add(b);

        Self {
            result: result.0,
            flags: Self::flags_for_operation(a, b, result)
        }
    }

    pub fn sub(a: u8, b: u8) -> Self {
        let result = a.overflowing_sub(b);

        Self {
            result: result.0,
            flags: Self::flags_for_operation(a, b, result)
        }
    }

    pub fn xor(a: u8, b: u8) -> Self {
        let result = a ^ b;

        Self {
            result,
            flags: Self::flags_for_operation(a, b, (result, false))
        }
    }
}
