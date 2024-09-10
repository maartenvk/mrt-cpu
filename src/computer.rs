use crate::types::Opcode;

pub struct System {
    rom: Vec<u8>,
    ram: Vec<u8>,
    regs: [u8;16],
    ip: u8,
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
            ip: 0
        }
    }

    pub fn get_regs(&self) -> [u8;16] {
        self.regs
    }

    pub fn get_ip(&self) -> u8 {
        self.ip
    }

    pub fn jump(&mut self, address: u8) {
        self.ip = address;
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
                let addition = reg2.unwrap().overflowing_add(*reg3.unwrap());
                self.regs[reg_raw] = addition.0;
                self.ip += 2;
            },
            Opcode::SB => {
                let mut_access = self.ram.get_mut(offset);
                if let Some(address) = mut_access {
                    *address = *reg.unwrap();
                } else {
                    println!("Error: out of bounds memory access [{:#06x}] ip={}", offset, self.ip);
                }

                self.ip += 2;
            },
            Opcode::LB => {
                let access = self.ram.get(offset);
                if let Some(address) = access {
                    self.regs[reg_raw] = *address;
                } else {
                    println!("Error: out of bounds memory access [{:#06x}] ip={}", offset, self.ip);
                }

                self.ip += 2;
            }
        };

        false
    }
}
