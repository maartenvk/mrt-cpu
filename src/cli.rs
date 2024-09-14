use std::{any::type_name, fs::File, path::Path, sync::{atomic::{AtomicBool, Ordering}, Arc}};

use crate::{compiler::Compiler, computer::System, types::Instruction};

pub struct Cli {
    system: System,
    interrupt: Arc<AtomicBool>
}

#[derive(Debug)]
pub enum CliError {
    MissingParameter(&'static str),
    InvalidParameterType(&'static str, &'static str),
    FailedParameterConstraint(&'static str),
    FailedToReadFromFile,
    FailedToWriteToFile,
    OperationError,
}

impl Cli {
    pub fn new(interrupt: Arc<AtomicBool>) -> Self {
        Self {
            system: System::new(64),
            interrupt
        }
    }

    fn unpack<T: std::str::FromStr>(param_name: &'static str, string: &str) -> Result<T, CliError> {
        if let Ok(n) = string.parse::<T>() {
            Ok(n)
        } else {
            Err(CliError::InvalidParameterType(param_name, type_name::<T>()))
        }
    }

    pub fn load_rom(&mut self, command: Vec<&str>) -> Result<(), CliError> {
        let path = command.get(1);
        if path.is_none() {
            return Err(CliError::MissingParameter(stringify!(path)));
        }

        let rom = std::fs::read(Path::new(path.unwrap())); 
        if rom.is_err() {
            return Err(CliError::FailedToReadFromFile);
        }

        let result = self.system.load_rom(rom.unwrap());
        if result.is_err() {
            println!("Cli Operation Error: {:?}", result);
            return Err(CliError::OperationError);
        }

        Ok(())
    }

    pub fn ram_size(&mut self, command: Vec<&str>) -> Result<(), CliError> {
        let size = command.get(1);
        if size.is_none() {
            return Err(CliError::MissingParameter(stringify!(size)));
        }

        let size = Self::unpack::<usize>(stringify!(size), size.unwrap())?;
        let result = self.system.load_ram(vec![0; size]);
        if result.is_err() {
            println!("Cli Operation Error: {:?}", result);
            return Err(CliError::OperationError);
        }

        Ok(())
    }

    pub fn step(&mut self, command: Vec<&str>) -> Result<(), CliError> {
        let step_count = command.get(1);
        let mut step_count = if step_count.is_some() {
            Self::unpack::<usize>(stringify!(step_count), step_count.unwrap())?
        } else { 1 };
    
        while step_count > 0 {
            step_count -=1 ;
            if self.system.tick() {
                break;
            }
        }

        Ok(())
    }

    pub fn continue_exec(&mut self) -> Result<(), CliError> {
        const CHECK_INTERVAL: usize = 1000;
        let mut counter = 0;

        loop {
            if self.system.tick() {
                break;
            }

            counter += 1;
            if counter > CHECK_INTERVAL {
                counter = 0;
                if self.interrupt.load(Ordering::Acquire) {
                    break;
                }
            }
        }

        self.interrupt.store(false, Ordering::Release);
        Ok(())
    }

    pub fn compile(&mut self, command: Vec<&str>) -> Result<(), CliError> {
        let input_path = command.get(1);
        if input_path.is_none() {
            return Err(CliError::MissingParameter(stringify!(input_path)));
        }

        let output_path = command.get(2);
        let output_path = output_path.unwrap_or(&"out.rom");

        let input_file = File::open(Path::new(input_path.unwrap())); 
        if input_file.is_err() {
            return Err(CliError::FailedToReadFromFile);
        }

        let output_file = File::create(Path::new(output_path));
        if output_file.is_err() {
            return Err(CliError::FailedToWriteToFile);
        }

        let mut compiler = Compiler::new(input_file.unwrap(), output_file.unwrap());
        let result = compiler.compile();
        if result.is_ok() {
            println!("Info: Compilation succesful, written to file: {}", output_path);
            Ok(())
        } else {
            println!("Error: Compilation failed: {:?}", result.err().unwrap());
            Err(CliError::OperationError)
        }
    }

    pub fn print_regs(&self) -> Result<(), CliError> {
        let regs = self.system.get_regs();
        for y in 0..4 {
            for x in 0..4 {
                let idx = y * 4 + x;

                if idx < 10 {
                    print!(" ");
                }

                print!("r{} = {:#04x} ", idx, regs[idx])
            }

            match y {
                0 => print!("\tip={:#04x}", self.system.get_ip()),
                1 => {
                    print!("\t");
                    for flag in self.system.get_flags_register().get_flags() {
                        print!("{}", flag);
                    }
                },
                _ => {}
            }

            println!();
        }

        Ok(())
    }

    pub fn goto(&mut self, command: Vec<&str>) -> Result<(), CliError> {
        let address = command.get(1);
        if address.is_none() {
            return Err(CliError::MissingParameter(stringify!(address)));
        }

        let address = Self::unpack::<u8>(stringify!(address), address.unwrap())?;
        self.system.jump(address);
        Ok(())
    }

    fn disassemble_single(&self, ip: u16) -> u16 {
        let first_byte = self.system.get_rom(ip);
        let second_byte = self.system.get_rom(ip + 1);

        let generated = Instruction::disassemble(first_byte, second_byte);
    
        if let Ok(instruction) = generated {
            println!("{:#04x}: {}", ip, instruction);
            
            Instruction::get_length(match instruction {
                Instruction::NoParam(opcode) => opcode,
                Instruction::RegImm(opcode, _, _) => opcode,
                Instruction::DoubleReg(opcode, _, _) => opcode,
                Instruction::TripleReg(opcode, _, _, _) => opcode
            })
        } else {
            println!("Error: Disassembly failed: {:?}", generated.unwrap_err());
            1
        }
    }

    pub fn disassemble(&self, command: Vec<&str>) -> Result<(), CliError> {
        let from = command.get(1);
        let to = command.get(2);

        if let Some(to) = to {
            let from = Self::unpack::<u16>(stringify!(from), from.unwrap())?;
            let to = Self::unpack::<u16>(stringify!(to), to)?;

            if from > to {
                return Err(CliError::FailedParameterConstraint(stringify!(from > to)))
            }

            let mut ip = from as u16;
            while ip < to {
                ip += self.disassemble_single(ip);
            }
        } else if let Some(count) = from {
            let count = Self::unpack::<u16>(stringify!(count), count)?;
            let mut ip = self.system.get_ip() as u16;
            for _ in 0..count {
                ip += self.disassemble_single(ip);
            }
        } else {
            _ = self.disassemble_single(self.system.get_ip() as u16);
        }

        Ok(())
    }

    pub fn read_memory(&self, command: Vec<&str>) -> Result<(), CliError> {
        let address = command.get(1);
        if address.is_none() {
            return Err(CliError::MissingParameter(stringify!(address)));
        }

        let mut address = Self::unpack::<u16>(stringify!(address), address.unwrap())?;
        
        let count = command.get(2);
        if let Some(count) = count {
            let count = Self::unpack::<usize>(stringify!(count), count)?;
            
            for _ in 0..count {
                let value = self.system.get_mem(address);
                println!("[{:#06x}: {:#04x}]", address, value);
                address += 1;
            }
        } else {
            let value = self.system.get_mem(address);
            println!("[{:#06x}]: {:#04x}", address, value);
        }
        
        Ok(())
    }

    pub fn write_memory(&mut self, command: Vec<&str>) -> Result<(), CliError> {
        let address = command.get(1);
        if address.is_none() {
            return Err(CliError::MissingParameter(stringify!(address)));
        }

        let byte = command.get(2);
        if byte.is_none() {
            return Err(CliError::MissingParameter(stringify!(byte)));
        }

        let mut address = Self::unpack::<u16>(stringify!(address), address.unwrap())?;
        let byte = Self::unpack::<u8>(stringify!(byte), byte.unwrap())?;

        let count = command.get(3);
        if let Some(count) = count {
            let count = Self::unpack::<usize>(stringify!(count), count)?;
            
            for _ in 0..count {
                self.system.set_mem(address, byte);
                address += 1;
            }
        } else {
            self.system.set_mem(address, byte);
        }
        
        Ok(())
    }
}
