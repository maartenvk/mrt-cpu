use std::fmt::{Display, Write};

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum Flags {
    Zero,
    Carry,
    Sign,
    Overflow,
}

impl Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Flags::Zero => 'Z',
            Flags::Carry => 'C',
            Flags::Sign => 'S',
            Flags::Overflow => 'O',
        })
    }
}

pub struct FlagsRegister {
    flags: [bool; 4],
}

impl FlagsRegister {
    pub fn new() -> Self {
        Self { flags: [false; 4] }
    }

    pub fn set(&mut self, flag: Flags) {
        self.flags[flag as usize] = true;
    }

    pub fn unset(&mut self, flag: Flags) {
        self.flags[flag as usize] = false;
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
