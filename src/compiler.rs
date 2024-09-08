use std::{fs::File, io::{Read, Write}};

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Opcode {
    HLT,
    LDI
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
            _ => return Err(OpcodeConversionError::NoSuchOpcode)
        };

        Ok(result)
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum Register {
    r0,
    r1,
    r2
}

#[derive(Debug)]
pub enum RegisterConversionError {
    NoSuchRegister
}

impl TryFrom<&str> for Register {
    type Error = RegisterConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let result = match value.to_lowercase().as_str() {
            "r0" => Register::r0,
            "r1" => Register::r1,
            "r2" => Register::r2,
            _ => return Err(RegisterConversionError::NoSuchRegister)
        };

        Ok(result)
    }
}

pub enum Instruction {
    NoParam(Opcode),
    RegImm(Opcode, Register, u8),
    TripleReg(Opcode, Register, Register, Register)
}

impl Instruction {
    pub fn serialize(&self) -> Vec<u8> {
        return vec![*match self {
            Self::NoParam(opcode)
                => opcode,
            Self::RegImm(opcode, _, _)
                => opcode,
            Self::TripleReg(opcode, _, _, _)
                => opcode
        } as u8];
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
    InvalidNumber(String)
}

#[derive(Debug, Clone)]
pub enum CompilationState {
    Comment(Vec<char>),
    Symbol(Vec<char>),
    Numeric(Vec<char>),
}

#[derive(Debug)]
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

        for token in result.unwrap() {
            println!("Token -> {:?}", token);
        }

        // flush to output
        let binary = &self.generated.create_binary();
        if self.output_file.write_all(binary).is_err() {
            return Err(CompileError::WriteToOutputFailed);
        }

        Ok(())
    }
}
