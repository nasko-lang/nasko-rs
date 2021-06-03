use crate::instructions::OpCode;
use uuid::Uuid;
use chrono::prelude::*;
use num_cpus;

#[derive(Clone, Debug)]
pub enum VMEventType {
  Start,
  GracefulStop { code: u32 },
  Crash { code: u32 }
}

impl VMEventType {
  pub fn stop_code(&self) -> u32 {
    match &self {
      VMEventType::Start => 0,
      VMEventType::GracefulStop { code } => *code,
      VMEventType::Crash { code }  => *code
    }
  }
}

#[derive(Clone, Debug)]
pub struct VMEvent {
  pub event: VMEventType,
  pub at: DateTime<Utc>,
  application_id: Uuid
}

pub const DEFAULT_HEAP_SIZE: usize = 64;
pub const DEFAULT_STACK_SIZE: usize = 1024 * 1024 * 2;

#[derive(Debug, Default, Clone)]
pub struct VM {
  pub registers: [i32; 32],
  pub float_registers: [f64; 32],
  pub program: Vec<u8>,
  pub logical_cores: usize,
  pub id: Uuid,
  pub remainder: usize,
  pub eq_flag: bool,
  pc: usize,
  heap: Vec<u8>,
  stack: Vec<i32>,
  events: Vec<VMEvent>,
}

impl VM {
  pub fn prepend_header(b: Vec<u8>) -> Vec<u8> {
    [vec![78, 83, 75, 79], b].concat() // NSKO
  }

  pub fn verify_header(&mut self) -> bool {
    self.program[0..4] == [78, 83, 75, 79]
  }

  pub fn new() -> VM {
    VM {
      registers: [0; 32],
      float_registers: [0.; 32],
      pc: 0,
      program: vec![],
      logical_cores: num_cpus::get(),
      remainder: 0,
      eq_flag: false,
      id: Uuid::new_v4(),
      heap: vec![0; DEFAULT_HEAP_SIZE],
      stack: Vec::with_capacity(DEFAULT_STACK_SIZE),
      events: Vec::new()
    }
  }

  pub fn run(&mut self) -> Vec<VMEvent> {
    self.events.push(VMEvent {
      event: VMEventType::Start,
      at: Utc::now(),
      application_id: self.id
    });

    if !self.verify_header() {
      self.events.push(VMEvent {
        event: VMEventType::Crash { code: 1 },
        at: Utc::now(),
        application_id: self.id
      });
      println!("Header was incorrect");
      return self.events.clone();
    }

    self.pc += 4;

    loop {
      if self.pc >= self.program.len() && self.pc < 4 {
        self.events.push(VMEvent {
          event: VMEventType::Crash { code: 2 },
          at: Utc::now(),
          application_id: self.id
        });
        return self.events.clone();
      } else if self.pc >= self.program.len() {
        self.events.push(VMEvent {
          event: VMEventType::GracefulStop { code: 1 },
          at: Utc::now(),
          application_id: self.id
        });
        return self.events.clone();
      }

      match self.decode_opcode() {
        OpCode::Halt => {
          self.events.push(VMEvent {
            event: VMEventType::GracefulStop { code: 0 },
            at: Utc::now(),
            application_id: self.id
          });
          return self.events.clone()
        },
        OpCode::Load => {
          let register = self.next_8_bits() as usize;
          let value = self.next_16_bits() as u16;
          
          self.set_register(register, value as f64);

          continue;
        },
        OpCode::Add => {
          let (r1, r2, r3) = (self.next_8_bits() as usize, self.next_8_bits() as usize, self.next_8_bits() as usize);
          let (r1, r2, r3) = (self.get_register(r1), self.get_register(r2), r3);
          
          self.set_register(r3, r1 + r2);
        }
        OpCode::Sub => {
          let (r1, r2, r3) = (self.next_8_bits() as usize, self.next_8_bits() as usize, self.next_8_bits() as usize);
          let (r1, r2, r3) = (self.get_register(r1), self.get_register(r2), r3);

          self.set_register(r3, r1 - r2);
        },
        OpCode::Mul => {
          let (r1, r2, r3) = (self.next_8_bits() as usize, self.next_8_bits() as usize, self.next_8_bits() as usize);
          let (r1, r2, r3) = (self.get_register(r1), self.get_register(r2), r3);

          self.set_register(r3, r1 * r2);
        },
        OpCode::Div => {
          let (r1, r2, r3) = (self.next_8_bits() as usize, self.next_8_bits() as usize, self.next_8_bits() as usize);
          let (r1, r2, r3) = (self.get_register(r1), self.get_register(r2), r3);

          self.set_register(r3, r1 / r2);
          self.remainder = (r1 % r2) as usize;
        },
        OpCode::Eq => {
          let (r1, r2) = (self.next_8_bits() as usize, self.next_8_bits() as usize);
          let (r1, r2) = (self.get_register(r1), self.get_register(r2));

          self.eq_flag = r1 == r2;
        },
        OpCode::Neq => {
          let (r1, r2) = (self.next_8_bits() as usize, self.next_8_bits() as usize);
          let (r1, r2) = (self.get_register(r1), self.get_register(r2));

          self.eq_flag = r1 != r2;
        },
        OpCode::Jmp => {
          let target = self.registers[self.next_8_bits() as usize];
          self.pc = target as usize;
        },
        OpCode::JmpF => {
          let target = self.registers[self.next_8_bits() as usize] as usize;
          self.pc += target;
        },
        OpCode::JmpB => {
          let target = self.registers[self.next_8_bits() as usize] as usize;
          self.pc -= target;
        },
        OpCode::Gt => {
          let (r1, r2) = (self.next_8_bits() as usize, self.next_8_bits() as usize);
          let (r1, r2) = (self.get_register(r1), self.get_register(r2));

          self.eq_flag = r1 > r2;
        },
        OpCode::Gte => {
          let (r1, r2) = (self.next_8_bits() as usize, self.next_8_bits() as usize);
          let (r1, r2) = (self.get_register(r1), self.get_register(r2));

          self.eq_flag = r1 >= r2;
        },
        OpCode::Lt => {
          let (r1, r2) = (self.next_8_bits() as usize, self.next_8_bits() as usize);
          let (r1, r2) = (self.get_register(r1), self.get_register(r2));

          self.eq_flag = r1 < r2;
        },
        OpCode::Lte => {
          let (r1, r2) = (self.next_8_bits() as usize, self.next_8_bits() as usize);
          let (r1, r2) = (self.get_register(r1), self.get_register(r2));

          self.eq_flag = r1 <= r2;
        },
        _ => {
          // println!("Illegal Opcode ({:#?})", self.program[self.pc - 1]);
          self.events.push(VMEvent {
            event: VMEventType::Crash { code: 3 },
            at: Utc::now(),
            application_id: self.id
          });
          return self.events.clone();
        }
      }
    }
  }

  fn set_register(&mut self, register: usize, value: f64) {
    if register <= 32 && register >= 63 {
      self.float_registers[register - 32] = value;
    } else if register <= 31 {
      self.registers[register] = value as i32;
    } else {
      println!("Register not found ({})!", register);
    }
  }

  fn get_register(&mut self, register: usize) -> f64 {
    if register <= 32 && register >= 63 {
      self.float_registers[register - 32]
    } else if register <= 31 {
      self.registers[register] as f64
    } else {
      0.
    }
  }

  fn next_8_bits(&mut self) -> u8 {
    let bits = self.program[self.pc];
    self.pc += 1;
    return bits;
  }

  fn next_16_bits(&mut self) -> u16 {
    let bits = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
    self.pc += 2;
    return bits;
  }

  fn decode_opcode(&mut self) -> OpCode {
    let opcode = OpCode::from(self.program[self.pc]);
    self.pc += 1;
    return opcode;
  }
}
