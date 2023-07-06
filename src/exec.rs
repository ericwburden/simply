use crate::parse::Instruction;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
enum ExecErrorKind {
    UninitializedRegister(String),
    NegativeExecutionPointer,
}

impl fmt::Display for ExecErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UninitializedRegister(s) => write!(f, "register '{}' is uninitialized", s),
            Self::NegativeExecutionPointer => write!(f, "execution pointer less than zero"),
        }
    }
}

impl std::error::Error for ExecErrorKind {}

#[derive(Debug)]
pub(crate) struct ExecError {
    source: ExecErrorKind,
}

impl ExecError {
    fn new(source: ExecErrorKind) -> Self {
        Self { source }
    }
}

impl fmt::Display for ExecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to execute instruction, {}", self.source)
    }
}

impl std::error::Error for ExecError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

type Result<T> = std::result::Result<T, ExecError>;

#[derive(Default)]
pub(crate) struct Program {
    memory: HashMap<String, i32>,
    pointer: usize,
}

impl Program {
    pub(crate) fn run(&mut self, instructions: Vec<Instruction>) -> Result<()> {
        while self.pointer < instructions.len() {
            instructions[self.pointer].execute(self)?;
        }
        Ok(())
    }
}

trait Execute {
    fn execute(&self, program: &mut Program) -> Result<()>;
}

impl Execute for Instruction {
    fn execute(&self, program: &mut Program) -> Result<()> {
        match self {
            Instruction::Set { register, value } => {
                program.memory.insert(register.clone(), *value);
                program.pointer += 1;
            }
            Instruction::Cpy {
                register1,
                register2,
            } => {
                let value = *program.memory.get(register1).unwrap();
                match program.memory.get_mut(register2) {
                    Some(v) => *v = value,
                    None => {
                        program.memory.insert(register2.clone(), value);
                    }
                };
                program.pointer += 1;
            }
            Instruction::Add {
                register1,
                register2,
            } => {
                let value = *program.memory.get(register1).ok_or_else(|| ExecError::new(
                    ExecErrorKind::UninitializedRegister(register1.to_owned()),
                ))?;
                let add_to = program.memory.get_mut(register2).ok_or_else(|| ExecError::new(
                    ExecErrorKind::UninitializedRegister(register2.to_owned()),
                ))?;
                *add_to += value;
                program.pointer += 1;
            }
            Instruction::Sub {
                register1,
                register2,
            } => {
                let value = *program.memory.get(register1).ok_or_else(|| ExecError::new(
                    ExecErrorKind::UninitializedRegister(register1.to_owned()),
                ))?;
                let sub_from = program.memory.get_mut(register2).ok_or_else(|| ExecError::new(
                    ExecErrorKind::UninitializedRegister(register2.to_owned()),
                ))?;
                *sub_from -= value;
                program.pointer += 1;
            }
            Instruction::Out { register } => {
                let value = *program.memory.get(register).ok_or_else(|| ExecError::new(
                    ExecErrorKind::UninitializedRegister(register.to_owned()),
                ))?;
                println!("{}", value);
                program.pointer += 1;
            }
            Instruction::Jmp { register } => {
                let target = *program.memory.get(register).ok_or_else(|| ExecError::new(
                    ExecErrorKind::UninitializedRegister(register.to_owned()),
                ))?;
                if target < 1 {
                    return Err(ExecError::new(ExecErrorKind::NegativeExecutionPointer));
                }
                program.pointer = (target - 1) as usize;
            }
            Instruction::Jwz {
                register1,
                register2,
            } => {
                let value = program.memory.get(register1).unwrap();
                if *value == 0 {
                    let target = *program.memory.get(register2).ok_or_else(|| ExecError::new(
                        ExecErrorKind::UninitializedRegister(register2.to_owned()),
                    ))?;
                    if target < 1 {
                        return Err(ExecError::new(ExecErrorKind::NegativeExecutionPointer));
                    }
                    program.pointer = (target - 1) as usize;
                } else {
                    program.pointer += 1;
                }
            }
            Instruction::Jnz {
                register1,
                register2,
            } => {
                let value = program.memory.get(register1).unwrap();
                if *value == 0 {
                    program.pointer += 1;
                } else {
                    let target = *program.memory.get(register2).ok_or_else(|| ExecError::new(
                        ExecErrorKind::UninitializedRegister(register2.to_owned()),
                    ))?;
                    if target < 1 {
                        return Err(ExecError::new(ExecErrorKind::NegativeExecutionPointer));
                    }
                    program.pointer = (target - 1) as usize;
                }
            }
            Instruction::Jwn {
                register1,
                register2,
            } => {
                let value = program.memory.get(register1).unwrap();
                if *value < 0 {
                    let target = *program.memory.get(register2).ok_or_else(|| ExecError::new(
                        ExecErrorKind::UninitializedRegister(register2.to_owned()),
                    ))?;
                    if target < 1 {
                        return Err(ExecError::new(ExecErrorKind::NegativeExecutionPointer));
                    }
                    program.pointer = (target - 1) as usize;
                } else {
                    program.pointer += 1;
                }
            }
            Instruction::Jwp {
                register1,
                register2,
            } => {
                let value = program.memory.get(register1).unwrap();
                if *value > 0 {
                    let target = *program.memory.get(register2).ok_or_else(|| ExecError::new(
                        ExecErrorKind::UninitializedRegister(register2.to_owned()),
                    ))?;
                    if target < 1 {
                        return Err(ExecError::new(ExecErrorKind::NegativeExecutionPointer));
                    }
                    program.pointer = (target - 1) as usize;
                } else {
                    program.pointer += 1;
                }
            }
            Instruction::Gth {
                register1,
                register2,
            } => {
                let value = *program.memory.get(register1).ok_or_else(|| ExecError::new(
                    ExecErrorKind::UninitializedRegister(register1.to_owned()),
                ))?;
                let target = program.memory.get_mut(register2).ok_or_else(|| ExecError::new(
                    ExecErrorKind::UninitializedRegister(register2.to_owned()),
                ))?;
                if value > *target {
                    *target = 1;
                } else {
                    *target = -1;
                }
                program.pointer += 1;
            }
            Instruction::Lth {
                register1,
                register2,
            } => {
                let value = *program.memory.get(register1).ok_or_else(|| ExecError::new(
                    ExecErrorKind::UninitializedRegister(register1.to_owned()),
                ))?;
                let target = program.memory.get_mut(register2).ok_or_else(|| ExecError::new(
                    ExecErrorKind::UninitializedRegister(register2.to_owned()),
                ))?;
                if value < *target {
                    *target = 1;
                } else {
                    *target = -1;
                }
                program.pointer += 1;
            }
            Instruction::Chr { register } => {
                let value = *program.memory.get(register).ok_or_else(|| ExecError::new(
                    ExecErrorKind::UninitializedRegister(register.to_owned()),
                ))?;
                let out_char = u32::try_from(value).ok().and_then(char::from_u32).unwrap_or('·');
                print!("{out_char}");
                program.pointer += 1;
            }
            
        }
        Ok(())
    }
}
