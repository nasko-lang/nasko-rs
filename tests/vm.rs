use nasko::vm::*;

#[test]
fn vm_init() {
  let vm = VM::new();
  assert_eq!(vm.registers[0], 0, "registers[0] should be 0");
  assert_eq!(vm.program, vec![], "program should be empty");
  // assert_eq!(vm.pc, 0, "program counter should be 0");
  assert_eq!(vm.eq_flag, false, "equal flag must be false");
  assert_eq!(vm.remainder, 0, "remainder should be 0");
  assert_eq!(vm.float_registers[0], 0., "float_registers[0] should be 0 (as f64)")
}

#[test]
fn vm_run_illegal() {
  let bytecode = vec![
    78, 83, 75, 79,
    254, 0, 0, 0
  ];
  let mut vm = VM::new();

  vm.program = bytecode;
  vm.run();
}

#[test]
fn vm_run_halt() {
  let bytecode = vec![
    78, 83, 75, 79,
    0, 0, 0, 0
  ];
  let mut vm = VM::new();

  vm.program = bytecode;
  vm.run(); // should halt without issue
}

#[test]
fn vm_run_load() {
  let bytecode = vec![
    78, 83, 75, 79,
    1, 0, 0, 30
  ];
  let mut vm = VM::new();

  vm.program = bytecode;
  vm.run();

  assert_eq!(vm.registers[0], 30, "vm register[0] should be set to 30");
}

#[test]
fn vm_run_add() {
  let bytecode = vec![
    78, 83, 75, 79, // Header
    1, 0, 0, 30, // Load 30 into 0
    1, 1, 0, 30, // Load 30 into 1
    2, 0, 1,  2, // Store Addition Result in 2
  ];
  let mut vm = VM::new();

  vm.program = bytecode;
  vm.run();

  assert_eq!(vm.registers[2], 60, "vm register[2] should be set to 60");
}

#[test]
fn vm_run_sub() {
  let bytecode = vec![
    78, 83, 75, 79, // Header
    1, 0, 0, 30, // Load 30 into 0
    1, 1, 0, 15, // Load 15 into 1
    3, 0, 1,  2, // Store Subtraction Result in 2
  ];
  let mut vm = VM::new();

  vm.program = bytecode;
  vm.run();

  assert_eq!(vm.registers[2], 15, "vm register[2] should be set to 15");
}

#[test]
fn vm_run_mul() {
  let bytecode = vec![
    78, 83, 75, 79, // Header
    1, 0, 0,  2, // Load 2 into 0
    1, 1, 0, 15, // Load 15 into 0
    4, 0, 1,  2, // Store Multiplication Result in 2
  ];
  let mut vm = VM::new();

  vm.program = bytecode;
  vm.run();

  assert_eq!(vm.registers[2], 30, "vm register[2] should be set to 30");
}

#[test]
fn vm_run_div() {
  let bytecode = vec![
    78, 83, 75, 79, // Header
    1, 0, 0, 61, // Load 61 into 0
    1, 1, 0,  2, // Load 2 into 0
    5, 0, 1,  2, // Store Division Result in 2
  ];
  let mut vm = VM::new();

  vm.program = bytecode;
  vm.run();

  assert_eq!(vm.registers[2], 30, "vm register[2] should be set to 30");
  assert_eq!(vm.remainder, 1, "vm remainder should be set to 1");
}

#[test]
fn vm_run_eq() {
  let bytecode = vec![
    78, 83, 75, 79, // Header
    1, 0, 0, 1, // Load 1 into 0
    1, 1, 0, 1, // Load 1 into 1
    6, 0, 1, 0, // Check if r0 == r1
  ];
  let mut vm = VM::new();

  vm.program = bytecode;
  vm.run();

  assert_eq!(vm.eq_flag, true, "vm eq_flag should be set to true");
}

#[test]
fn vm_run_neq() {
  let bytecode = vec![
    78, 83, 75, 79, // Header
    1, 0, 0, 1, // Load 1 into 0
    1, 1, 0, 1, // Load 1 into 1
    7, 0, 1, 0, // Check if r0 != r1
  ];
  let mut vm = VM::new();

  vm.program = bytecode;
  vm.run();

  assert_eq!(vm.eq_flag, false, "vm eq_flag should be set to true");
}

#[test]
fn vm_run_jmp() {
  let bytecode = vec![
    78, 83, 75, 79, // Header
    1, 1, 0, 16, // Load 16 into 1
    8, 1, 0,  0, // Jump to addr 12 (register 1) 
    1, 3, 0,  1, // Load 1 into 3
    1, 2, 0,  9, // Load 9 into 2
  ];
  let mut vm = VM::new();

  vm.program = bytecode;
  vm.run();

  assert_eq!(vm.registers[2], 9, "vm registers[2] should be set to 9");
  assert_eq!(vm.registers[3], 0, "vm registers[3] should be set to 0");
}

#[test]
fn vm_run_jmpf() {
  let bytecode = vec![
    78, 83, 75, 79, // Header
    1, 1, 0, 6, // Load 6 into 1
    9, 1, 0, 0, // Jump Forward by 6 (register 1)
    1, 3, 0, 1, // Load 1 into 3
    1, 2, 0, 9, // Load 9 into 2
  ];
  let mut vm = VM::new();

  vm.program = bytecode;
  vm.run();

  assert_eq!(vm.registers[2], 9, "vm registers[2] should be set to 9");
  assert_eq!(vm.registers[3], 0, "vm registers[3] should be set to 0");
}

#[test]
fn vm_run_jmpb() {
  let bytecode = vec![
    78, 83, 75, 79, // Header
     1, 0, 0, 10, // Load 10 into 0
     9, 0, 0,  0, // Jump Forward by 10 (register 0)
     1, 5, 0, 22, // Load 22 into 5
     9, 3, 0,  0, // Jump Forward by 14 (register 3)
     1, 2, 0, 18, // Load 18 into 2
     1, 3, 0, 14, // Load 14 into 3
    10, 2, 0,  0, // Jump Backward by 17 (register 2)
     1, 4, 0, 38, // Load 38 into 4
  ];
  let mut vm = VM::new();

  vm.program = bytecode;
  vm.run();

  assert_eq!(vm.registers[0], 10, "vm registers[0] should be set to 11");
  assert_eq!(vm.registers[2], 18, "vm registers[2] should be set to 18");
  assert_eq!(vm.registers[3], 14, "vm registers[3] should be set to 14");
  assert_eq!(vm.registers[4], 38, "vm registers[4] should be set to 38");
  assert_eq!(vm.registers[5], 22, "vm registers[5] should be set to 22");
}

#[test]
fn vm_run_gt() {
  let bytecode = vec![
    78, 83, 75, 79, // Header
     1, 0, 0, 10, // Load 10 into 0
     1, 1, 0,  9, // Load 9 into 0
    11, 0, 1,  0, // Check if (reg 0) > (reg 1)
  ];
  let mut vm = VM::new();

  vm.program = bytecode;
  vm.run();

  assert_eq!(vm.eq_flag, true, "vm eq_flag should be set to true");
}

#[test]
fn vm_run_gte() {
  let bytecode = vec![
    78, 83, 75, 79, // Header
     1, 0, 0, 10, // Load 10 into 0
     1, 1, 0, 10, // Load 10 into 0
    12, 0, 1,  0, // Check if (reg 0) >= (reg 1)
  ];
  let mut vm = VM::new();

  vm.program = bytecode;
  vm.run();

  assert_eq!(vm.eq_flag, true, "vm eq_flag should be set to true");
}

#[test]
fn vm_run_lt() {
  let bytecode = vec![
    78, 83, 75, 79, // Header
     1, 0, 0,  9, // Load 9 into 0
     1, 1, 0, 10, // Load 10 into 0
    13, 0, 1,  0, // Check if (reg 0) < (reg 1)
  ];
  let mut vm = VM::new();

  vm.program = bytecode;
  vm.run();

  assert_eq!(vm.eq_flag, true, "vm eq_flag should be set to true");
}

#[test]
fn vm_run_lte() {
  let bytecode = vec![
    78, 83, 75, 79, // Header
     1, 0, 0, 10, // Load 10 into 0
     1, 1, 0, 10, // Load 10 into 0
    14, 0, 1,  0, // Check if (reg 0) <= (reg 1)
  ];
  let mut vm = VM::new();

  vm.program = bytecode;
  vm.run();

  assert_eq!(vm.eq_flag, true, "vm eq_flag should be set to true");
}
