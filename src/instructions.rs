#[derive(Debug, PartialEq)]
pub enum OpCode {
  Halt,
  Load,
  Add,
  Sub,
  Mul,
  Div,
  Eq,
  Neq,
  Jmp,
  JmpF,
  JmpB,
  Gt,
  Gte,
  Lt,
  Lte,
  Illegal
}

impl From<u8> for OpCode {
  fn from(v: u8) -> Self {
    match v {
      0  => OpCode::Halt,
      1  => OpCode::Load,
      2  => OpCode::Add,
      3  => OpCode::Sub,
      4  => OpCode::Mul,
      5  => OpCode::Div,
      6  => OpCode::Eq,
      7  => OpCode::Neq,
      8  => OpCode::Jmp,
      9  => OpCode::JmpF,
      10 => OpCode::JmpB,
      11 => OpCode::Gt,
      12 => OpCode::Gte,
      13 => OpCode::Lt,
      14 => OpCode::Lte,
      _ => OpCode::Illegal
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
  opcode: OpCode,
}

impl Instruction {
  pub fn new(opcode: OpCode) -> Instruction {
    Instruction {
      opcode
    }
  }
}
