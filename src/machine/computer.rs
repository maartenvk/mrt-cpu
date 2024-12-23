use crate::{
    compiler::instruction::Instruction,
    machine::{
        alu as ALU,
        flags::{Flags, FlagsRegister},
        storage::{ReadableStorage, WritableStorage, RAM},
    },
};

use crate::types::Opcode;

use super::storage::FiniteStorage;

pub struct System {
    ram: RAM<u8>,
    regs: [u8; 16],
    ip: u16,
    flags: FlagsRegister,
}

#[derive(Debug)]
pub enum LoadRomError {
    EmptyRom(),
}

#[derive(Debug)]
pub enum LoadRamError {
    EmptyRam(),
}

impl System {
    pub fn new(ram_size: usize) -> Self {
        Self {
            ram: RAM::new(ram_size),
            regs: [0; 16],
            ip: 0,
            flags: FlagsRegister::new(),
        }
    }

    pub fn get_mem(&self, address: u16) -> u8 {
        if let Ok(value) = self.ram.get(address as usize) {
            return value;
        }

        println!(
            "Error: out of bounds memory store operation [{:#06x}] ip={}",
            address, self.ip
        );

        return 0;
    }

    pub fn set_mem(&mut self, address: u16, value: u8) {
        if address == 0 {
            // intercept [0] as serial out
            print!("{}", value as char);
            return;
        }

        if self.ram.set(address as usize, value).is_err() {
            println!(
                "Error: out of bounds memory load operation [{:#06x}] ip={}",
                address, self.ip
            );
        }
    }

    pub fn get_regs(&self) -> [u8; 16] {
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

    pub fn load_rom(&mut self, rom: Vec<u8>) -> Result<(), LoadRomError> {
        if rom.is_empty() {
            return Err(LoadRomError::EmptyRom());
        }

        let old_ram_size = self.ram.size();
        self.ram = RAM::from(rom);

        let new_ram_size = self.ram.size();
        if old_ram_size > new_ram_size {
            self.ram.resize(old_ram_size);
        }

        return Ok(());
    }

    pub fn load_ram(&mut self, ram: Vec<u8>) -> Result<(), LoadRamError> {
        if ram.is_empty() {
            return Err(LoadRamError::EmptyRam());
        }

        self.ram = RAM::from(ram);
        return Ok(());
    }

    fn alu_operation<F>(&mut self, destination_raw_reg: usize, a: u8, b: u8, operation: F)
    where
        F: Fn(u8, u8) -> ALU::Result,
    {
        let alu_result = operation(a, b);

        self.flags = alu_result.flags;
        self.regs[destination_raw_reg] = alu_result.value;
    }

    // returns true if halted
    pub fn tick(&mut self) -> bool {
        let first_byte = self.ram.get(self.ip as usize).unwrap_or(0);
        let data = self.ram.get(self.ip as usize + 1).unwrap_or(0);

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
        let imm4 = data & 0b1111;

        let offset = (*reg2.unwrap_or(&0) as usize) << 8 | *reg3.unwrap_or(&0) as usize;

        self.ip += Instruction::get_length(opcode);
        match opcode {
            Opcode::HLT => {
                self.ip -= Instruction::get_length(Opcode::HLT); // Undo goto next instruction
                println!("Info: halting at ip={}", self.ip);
                return true;
            }
            Opcode::ADD => self.alu_operation(reg_raw, *reg2.unwrap(), *reg3.unwrap(), ALU::add),
            Opcode::XOR => self.alu_operation(reg_raw, *reg2.unwrap(), *reg3.unwrap(), ALU::xor),
            Opcode::SUB => self.alu_operation(reg_raw, *reg2.unwrap(), *reg3.unwrap(), ALU::sub),
            Opcode::SHL => self.alu_operation(reg_raw, *reg2.unwrap(), imm4, ALU::shl),
            Opcode::SHR => self.alu_operation(reg_raw, *reg2.unwrap(), imm4, ALU::shr),
            Opcode::AND => self.alu_operation(reg_raw, *reg2.unwrap(), *reg3.unwrap(), ALU::and),
            Opcode::OR => self.alu_operation(reg_raw, *reg2.unwrap(), *reg3.unwrap(), ALU::or),
            Opcode::LDI => {
                self.regs[reg_raw] = imm;
            }
            Opcode::SB => {
                self.set_mem(offset as u16, *reg.unwrap());
            }
            Opcode::LB => {
                self.regs[reg_raw] = self.get_mem(offset as u16);
            }
            Opcode::JNZ => {
                let zf_set = self.flags.is_set(Flags::Zero);
                if !zf_set {
                    self.ip = ((*reg.unwrap() as u16) << 8) | *reg2.unwrap() as u16;
                }
            }
            Opcode::JAL => {
                let new_ip = offset as u16;

                self.regs[reg_raw] = (self.ip >> 8) as u8;
                self.regs[reg2_raw] = self.ip as u8;

                self.ip = new_ip;
            }
            Opcode::JC => {
                let cf_set = self.flags.is_set(Flags::Carry);
                if cf_set {
                    self.ip = ((*reg.unwrap() as u16) << 8) | *reg2.unwrap() as u16;
                }
            }
            Opcode::NOT => {
                self.regs[reg_raw] = !*reg2.unwrap();
            }
        };

        false
    }
}
