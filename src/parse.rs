use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, i32, multispace0},
    combinator::{all_consuming, into, recognize},
    error::Error,
    multi::many0_count,
    sequence::{pair, terminated, tuple},
    Finish, IResult,
};

/// Parser for Register Identifiers
///
/// A register can be identified by any sequence of alphanumeric and underscore
/// characters that starts with a letter. Register names may not start with a number
/// or an underscore.
///
/// ## Valid Identifiers
/// - "alpha"
/// - "bear_nap"
/// - "a11y"
/// - "r0ck_and_r0ll"
fn register(input: &str) -> IResult<&str, &str> {
    let alphanum_or_underscore_seq = many0_count(alt((alphanumeric1, tag("_"))));
    recognize(pair(alpha1, alphanum_or_underscore_seq))(input)
}

/// Parser for One Register Instruction Keywords
///
/// There is a subset of valid instructions in the form "<keyword> <register>". This
/// function parses the valid keywords for this form of instruction.
fn one_register_keyword(input: &str) -> IResult<&str, &str> {
    let tags = (tag("out"), tag("jmp"));
    alt(tags)(input)
}

/// Parser for One Register/One Value Instruction Keywords
///
/// There is a subset of valid instructions in the form "<keyword> <register> <value>".
/// This function parses the valid keywords for this form of instruction.
fn one_register_one_value_keyword(input: &str) -> IResult<&str, &str> {
    let tags = (tag("set"),);
    alt(tags)(input)
}

/// Parser for Two Register Instruction Keywords
///
/// There is a subset of valid instructions in the form "<keyword> <register> <register>".
/// This function parses the valid keywords for this form of instruction.
fn two_register_keyword(input: &str) -> IResult<&str, &str> {
    let tags = (
        tag("cpy"),
        tag("add"),
        tag("sub"),
        tag("jwz"),
        tag("jnz"),
        tag("gth"),
        tag("lth"),
    );
    alt(tags)(input)
}

/// Parser for One Register Instruction
///
/// There is a subset of valid instructions in the form "<keyword> <register>". This
/// function parses this form of instruction. Extra whitespace between instruction
/// parts is ignored.
fn one_register_instruction(input: &str) -> IResult<&str, Instruction> {
    into(all_consuming(tuple((
        terminated(one_register_keyword, multispace0),
        terminated(register, multispace0),
    ))))(input)
}

/// Parser for One Register/One Value Instruction
///
/// There is a subset of valid instructions in the form "<keyword> <register> <value>".
/// This function parses this form of instruction. Extra whitespace between instruction
/// parts is ignored.
fn one_register_one_value_instruction(input: &str) -> IResult<&str, Instruction> {
    into(all_consuming(tuple((
        terminated(one_register_one_value_keyword, multispace0),
        terminated(register, multispace0),
        terminated(i32, multispace0),
    ))))(input)
}

/// Parser for Two Register Instruction
///
/// There is a subset of valid instructions in the form "<keyword> <register> <register>".
/// This function parses this form of instruction. Extra whitespace between instruction
/// parts is ignored.
fn two_register_instruction(input: &str) -> IResult<&str, Instruction> {
    into(all_consuming(tuple((
        terminated(two_register_keyword, multispace0),
        terminated(register, multispace0),
        terminated(register, multispace0),
    ))))(input)
}

/// Parser for Instructions
///
/// This parser parses any valid form of instruction by relying on the instruction
/// parsers for all valid forms of an instruction.
fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        one_register_instruction,
        one_register_one_value_instruction,
        two_register_instruction,
    ))(input)
}

/// Representation of Instructions
#[derive(Debug)]
pub(crate) enum Instruction {
    Set {
        register: String,
        value: i32,
    },
    Cpy {
        register1: String,
        register2: String,
    },
    Add {
        register1: String,
        register2: String,
    },
    Sub {
        register1: String,
        register2: String,
    },
    Out {
        register: String,
    },
    Jmp {
        register: String,
    },
    Jwz {
        register1: String,
        register2: String,
    },
    Jnz {
        register1: String,
        register2: String,
    },
    Gth {
        register1: String,
        register2: String,
    },
    Lth {
        register1: String,
        register2: String,
    },
}

impl From<(&str, &str)> for Instruction {
    fn from(input: (&str, &str)) -> Self {
        let (instr, reg) = input;
        match instr {
            "out" => Instruction::Out {
                register: reg.to_string(),
            },
            "jmp" => Instruction::Jmp {
                register: reg.to_string(),
            },
            _ => unreachable!(),
        }
    }
}

impl From<(&str, &str, i32)> for Instruction {
    fn from(input: (&str, &str, i32)) -> Self {
        let (instr, reg, val) = input;
        match instr {
            "set" => Instruction::Set {
                register: reg.to_string(),
                value: val,
            },
            _ => unreachable!(),
        }
    }
}

impl From<(&str, &str, &str)> for Instruction {
    fn from(input: (&str, &str, &str)) -> Self {
        let (instr, reg1, reg2) = input;
        let r1_string = reg1.to_string();
        let r2_string = reg2.to_string();
        match instr {
            "cpy" => Instruction::Cpy {
                register1: r1_string,
                register2: r2_string,
            },
            "add" => Instruction::Add {
                register1: r1_string,
                register2: r2_string,
            },
            "sub" => Instruction::Sub {
                register1: r1_string,
                register2: r2_string,
            },
            "jwz" => Instruction::Jwz {
                register1: r1_string,
                register2: r2_string,
            },
            "jnz" => Instruction::Jnz {
                register1: r1_string,
                register2: r2_string,
            },
            "gth" => Instruction::Gth {
                register1: r1_string,
                register2: r2_string,
            },
            "lth" => Instruction::Lth {
                register1: r1_string,
                register2: r2_string,
            },
            _ => unreachable!(),
        }
    }
}

impl<'a> TryFrom<&'a str> for Instruction {
    type Error = Error<&'a str>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let (_, result) = instruction(input).finish()?;
        Ok(result)
    }
}

impl TryFrom<String> for Instruction {
    type Error = Box<dyn std::error::Error>;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        let (_, result) = instruction(&input).finish().map_err(|e| e.to_string())?;
        Ok(result)
    }
}
