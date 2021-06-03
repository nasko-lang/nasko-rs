pub mod vm;
pub mod instructions;

use chrono::{Local, TimeZone};
use vm::{VM, VMEvent, VMEventType};
use std::fs;
use std::path::Path;
use ansi_term::Colour::Red;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: nasko <bytecode path>");
        return;
    }

    let path: String = args[1].clone();

    println!("checking validity of path: '{}'", path);

    let path = Path::new(&path);

    // possibly adding the extension manually?
    if !path.exists() {
        eprintln!("{} file at path does not exist. You must provide a valid, existing path.", Red.paint("Error: "));
        return;
    }

    println!("parsing bytecode from file: {}", path.file_name().unwrap().to_str().unwrap());
    let bytecode = fs::read_to_string(path.to_str().unwrap())
        .expect("Something went wrong reading the file");

    let bytecode = bytecode.chars()
            .collect::<Vec<char>>()
            .chunks(3)
            .map(|c| c.iter().collect::<String>())
            .map(|b| b.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

    let mut vm = VM::new();

    vm.program = bytecode;
    let events = vm.run();

    describe_events(events.clone());

    pprint_registers(&vm);
    pprint_misc(&vm);
}

fn describe_events(events: Vec<VMEvent>) {
    print!("\n");

    for event in events.iter() {
        match event.event {
            VMEventType::Start => print!("The VM started on {} ", event.at.with_timezone(&Local.timestamp(0, 0).timezone()).format("%a, %d %b %Y %T%.3f")),
            VMEventType::GracefulStop { code } => print!("then gracefully stopped with type: {}", grace_description(code)),
            VMEventType::Crash { code } => print!("then crashed with error: {}", error_description(code))
        }
    }

    print!("\n");
}

fn grace_description(code: u32) -> &'static str {
    match code {
        0 => "Graceful Halt",
        1 => "End of Program",
        _ => "Unknown Stop Code"
    }
}

fn error_description(code: u32) -> &'static str {
    match code {
        1 => "Invalid Header",
        2 => "Incomplete Bytecode",
        3 => "Unknown Opcode",
        _ => "Unknown Error"
    }
}

fn pprint_registers(vm: &VM) {
    print!("\n");

    let mut i = 0;
    for register in vm.registers.iter() {
        print!("r{}: {}\t", i, register);

        if i % 8 == 0 && i != 0 {
            print!("\n");
        }

        i += 1;
    }

    i = 0;
    for register in vm.float_registers.iter() {
        print!("r{}: {}\t", i + 31, register);

        if i % 8 == 0 {
            print!("\n");
        }

        i += 1;
    }

    print!("\n");
}

fn pprint_misc(vm: &VM) {
    print!("\n");

    print!("rem: {}\t", vm.remainder);
    print!("equ: {}", vm.eq_flag);

    print!("\n");
}
