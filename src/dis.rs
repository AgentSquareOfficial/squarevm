use crate::isa::Op;
use std::fs;

pub fn disassemble_file(input_path: &str) {
    let bytes = fs::read(input_path).expect("Failed to read input file");

    if bytes.len() < 9 {
        panic!("File is too short to be a valid .tbc file");
    }

    if &bytes[0..4] != b"MVM\0" {
        panic!("Invalid magic header. Not a valid minivm bytecode file.");
    }

    if bytes[4] != 1 {
        panic!("Unsupported version: {}", bytes[4]);
    }

    let mut offset = 9;

    while offset < bytes.len() {
        let (op, consumed) = Op::decode(&bytes[offset..])
            .unwrap_or_else(|| panic!("Failed to decode instruction at byte offset {}", offset));

        match op {
            Op::Push(n) => println!("PUSH {}", n),
            Op::Load(slot) => println!("LOAD {}", slot),
            Op::Store(slot) => println!("STORE {}", slot),
            _ => println!("{}", format!("{:?}", op).to_uppercase()),
        }

        offset += consumed;
    }
}