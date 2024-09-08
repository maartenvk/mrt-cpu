pub struct System {
    rom: Vec<u8>,
    ram: Vec<u8>,
    regs: [u8;8],
    ipc: u8,
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

    pub fn load_rom(&mut self, rom: Vec<u8>) -> Result<(),&str> {
        if rom.is_empty() {
            return Err("Loaded rom should not be empty");
        }
        
        self.rom = rom;
        Ok(())
    }

    pub fn load_ram(&mut self, ram: Vec<u8>) -> Result<(),&str> {
        if ram.is_empty() {
            return Err("Loaded ram should not be empty");
        }

        self.ram = ram;
        Ok(())
    }

    // returns true if halted
    pub fn tick(&mut self) -> bool {
        let opcode = self.rom.get(self.ipc as usize).unwrap_or(&0);
        let data = self.rom.get(self.ipc as usize + 1).unwrap_or(&0);

        match opcode {
            0 => { // HLT
                println!("Info: halting at ip={}", self.ipc);
                return true;
            },
            _ => {
                println!("Unhandled opcode: {}", opcode);
            }
        }

        false
    }
}
