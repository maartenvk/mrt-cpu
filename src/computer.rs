use crate::types::Opcode;

pub struct System {
    rom: Vec<u8>,
    ram: Vec<u8>,
    regs: [u8;8],
    ipc: u8,
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
            regs: [0;8],
            ipc: 0
        }
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
        let first_byte = *self.rom.get(self.ipc as usize).unwrap_or(&0);
        let data = *self.rom.get(self.ipc as usize + 1).unwrap_or(&0);

        let opcode_raw = first_byte >> 4;
        let opcode = Opcode::try_from(opcode_raw);
        if opcode.is_err() {
            println!("Info: Illegal Instruction: {}", opcode_raw);
            return false;
        }

        let opcode = opcode.unwrap();

        let reg = || {
            first_byte & 0b1111
        };

        let reg2 = || {
            data >> 4
        };

        let reg3 = || {
            data & 0b1111
        };

        let imm = || {
            data
        };

        match opcode {
            Opcode::HLT => {
                println!("Info: halting at ip={}", self.ipc);
                return true;
            },
            _ => {
                println!("Unhandled opcode {:?} at ip={}", opcode, self.ipc);
            }
        }

        false
    }
}
