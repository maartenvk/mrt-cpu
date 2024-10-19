use crate::types::{Opcode, Register};

#[derive(Debug, Clone)]
pub enum Token {
    Opcode(Opcode),
    Register(Register),
    Immediate(u8),
}
