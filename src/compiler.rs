use std::{collections::VecDeque, fs::File, io::{Read, Write}};

use crate::types::*;

impl Instruction {
    pub fn serialize(&self) -> Vec<u8> {
        return match self {
            Self::NoParam(opcode)
                => vec![
                    (*opcode as u8) << 4
                ],
            Self::RegImm(opcode, reg, imm)
                => vec![
                    (*opcode as u8) << 4 | *reg as u8, *imm
                ],
            Self::TripleReg(opcode, reg, reg2, reg3)
                => vec![
                    (*opcode as u8) << 4 | *reg as u8, (*reg2 as u8) << 4 | *reg3 as u8
                ]
        };
    }

    pub fn generate<F>(opcode: Opcode, mut consumer: F) -> Result<Self, CompileError> where F: FnMut() -> Result<Token, CompileError> {
        let reg = |consume_token: &mut F| {
            match consume_token()? {
                Token::Register(reg) => Ok(reg),
                token => Err(CompileError::UnexpectedTokenType(token))
            }
        };

        let imm = |consume_token: &mut F| {
            match consume_token()? {
                Token::Immediate(val) => Ok(val),
                token => Err(CompileError::UnexpectedTokenType(token))
            }
        };

        let create_no_param = |opcode| {
            Instruction::NoParam(opcode)
        };

        let create_reg_imm = |opcode, consumer: &mut F| -> Result<Instruction, CompileError> {
            Ok(Instruction::RegImm(opcode, reg(consumer)?, imm(consumer)?))
        };

        Ok(match opcode {
            Opcode::HLT => create_no_param(opcode),
            Opcode::LDI => create_reg_imm(opcode, &mut consumer)?,
            _ => todo!()
        })
    }
}

pub struct Bytecode {
    instructions: Vec<Instruction>
}

impl Bytecode {
    pub fn new() -> Self {
        Self {
            instructions: vec![]
        }
    }

    pub fn create_binary(&self) -> Vec<u8> {
        let mut binary: Vec<u8> = vec![];
        for instruction in &self.instructions {
            binary.append(&mut instruction.serialize());
        }

        binary
    }
}

pub struct Compiler {
    input_file: File,
    output_file: File,
    generated: Bytecode,
    collected_states: Vec<CompilationState>,
    line_number: i32,
    data: Vec<char>
}

#[derive(Debug)]
pub enum CompileError {
    ReadFromInputFailed,
    WriteToOutputFailed,
    UnexpectedEOF,
    UnhandledState(CompilationState),
    UnknownSymbol(String),
    InvalidNumber(String),
    UnexpectedTokenType(Token)
}

#[derive(Debug, Clone)]
pub enum CompilationState {
    Comment(Vec<char>),
    Symbol(Vec<char>),
    Numeric(Vec<char>),
}

#[derive(Debug, Clone)]
pub enum Token {
    Opcode(Opcode),
    Register(Register),
    Immediate(u8)
}

impl Compiler {
    pub fn new(input_file: File, output_file: File) -> Self {
        Self {
            input_file,
            output_file,
            generated: Bytecode::new(),
            collected_states: vec![],
            line_number: 0,
            data: vec![]
        }
    }

    fn consume(&mut self, mut state: CompilationState, c: char) -> Result<Option<CompilationState>, CompileError> {
        match state {
            CompilationState::Comment(ref mut data) => {
                if c == '\n' {
                    self.collected_states.push(state);
                    return Ok(None);
                } else {
                    data.push(c);
                }
            },
            CompilationState::Symbol(ref mut data) => {
                if !c.is_ascii_alphanumeric() {
                    self.collected_states.push(state);
                    return Ok(None);
                } else {
                    data.push(c);
                }
            },
            CompilationState::Numeric(ref mut data) => {
                if !c.is_ascii_digit() || c == '.' && data.contains(&'.') {
                    self.collected_states.push(state);
                    return Ok(None);
                } else {
                    data.push(c);
                }
            },
            _ => return Err(CompileError::UnhandledState(state))
        }

        Ok(Some(state))
    }

    fn collect_states(&mut self) -> Result<(), CompileError> {
        let mut current_state: Option<CompilationState> = None;
        let mut i = 0;
        while i < self.data.len() {
            let c = self.data[i];

            if c == '\n' {
                self.line_number += 1;
            }

            if current_state.is_none() {
                match c {
                    '#'
                        => current_state = Some(CompilationState::Comment(vec![])),

                    _ if c.is_ascii_alphabetic()
                        => current_state = Some(CompilationState::Symbol(vec![c])),

                    _ if c.is_ascii_digit()
                        => current_state = Some(CompilationState::Numeric(vec![c])),

                    _ => {}
                };

                i += 1;
                continue;
            }

            let state = current_state.unwrap();
            let result = self.consume(state, c);
            if result.is_err() {
                return Err(result.unwrap_err());
            }

            current_state = result.unwrap();

            i += 1;
        }

        if current_state.is_none() {
            Ok(())
        } else {
            Err(CompileError::UnexpectedEOF)
        }
    }

    fn flatten_states(&mut self) -> Result<Vec<Token>, CompileError> {
        let mut tokens : Vec<Token> = vec![];

        for state in &self.collected_states {
            match state {
                CompilationState::Comment(data) => {
                    println!("Info: Comment: {}", String::from_iter(data));
                },
                CompilationState::Symbol(data) => {
                    let data_str = String::from_iter(data);
                    let result = Opcode::try_from(data_str.as_str());
                    if result.is_ok() {
                        tokens.push(Token::Opcode(result.unwrap()));
                        continue;
                    }

                    let result = Register::try_from(data_str.as_str());
                    if result.is_ok(){
                        tokens.push(Token::Register(result.unwrap()));
                        continue;
                    }

                    return Err(CompileError::UnknownSymbol(String::from_iter(data)));
                },
                CompilationState::Numeric(data) => {
                    let str = String::from_iter(data);
                    let number = str.parse::<u8>();
                    if number.is_err() {
                        return Err(CompileError::InvalidNumber(str))
                    }

                    tokens.push(Token::Immediate(number.unwrap()));
                },
                _ => return Err(CompileError::UnhandledState(state.clone()))
            }
        }

        Ok(tokens)
    }

    fn create_bytecode(&mut self, tokens: &mut VecDeque<Token>) -> Result<(), CompileError> {
        let mut control_token: Option<Token> = None;
        loop {
            if let Some(ref token) = control_token {
                if let Token::Opcode(opcode) = token {
                    let mut token_consumer = || {
                        if tokens.is_empty() {
                            Err(CompileError::UnexpectedEOF)
                        } else {
                            Ok(tokens.pop_front().unwrap())
                        }
                    };

                    let instruction = Instruction::generate(*opcode, &mut token_consumer)?;
                    println!("Info: generated {:?}", instruction);

                    self.generated.instructions.push(instruction);
                    control_token = None;

                    continue;
                }

                return Err(CompileError::UnexpectedTokenType(token.clone()));
            }

            if tokens.is_empty() {
                break;
            }

            control_token = Some(tokens.pop_front().unwrap());
        }
        
        Ok(())
    }

    pub fn compile(&mut self) -> Result<(), CompileError> {
        let mut data = String::new();
        if self.input_file.read_to_string(&mut data).is_err() {
            return Err(CompileError::ReadFromInputFailed);
        }

        self.data = data.chars().collect::<Vec<char>>();

        let result = self.collect_states();
        if result.is_err() {
            return Err(result.unwrap_err());
        }

        let result = self.flatten_states();
        if result.is_err() {
            return Err(result.unwrap_err());
        }

        let tokens = result.unwrap();
        let result = self.create_bytecode(&mut VecDeque::from(tokens));
        if result.is_err() {
            return Err(result.unwrap_err());
        }

        // flush to output
        let binary = &self.generated.create_binary();
        if self.output_file.write_all(binary).is_err() {
            return Err(CompileError::WriteToOutputFailed);
        }

        Ok(())
    }
}
