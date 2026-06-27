use crate::isa::Op;
use std::fs;

fn parse_line(line: &str) -> Option<Op> {

    let mut code = line;

    if let Some((before_comment, _)) = line.split_once(';') {
        code = before_comment;
    }

    let code = code.trim();

    if code.is_empty() {
        return None;
    }

    let mut words = code.split_whitespace();

    let instruction = words.next()?;
    let instruction = instruction.to_uppercase();

    match instruction.as_str() {
        "PUSH" => {
            let value = words.next().expect("PUSH needs a value");
            let value = value.parse::<i64>().expect("Invalid number");
            Some(Op::Push(value))
        }

        "POP" => Some(Op::Pop),

        "DUP" => Some(Op::Dup),

        "SWAP" => Some(Op::Swap),

        "ADD" => Some(Op::Add),

        "SUB" => Some(Op::Sub),

        "MUL" => Some(Op::Mul),

        "DIV" => Some(Op::Div),

        "MOD" => Some(Op::Mod),

        "NEG" => Some(Op::Neg),

        "LOAD" => {
            let slot = words.next().expect("LOAD needs a slot");
            let slot = slot.parse::<u8>().expect("Invalid slot");
            Some(Op::Load(slot))
        }

        "STORE" => {
            let slot = words.next().expect("STORE needs a slot");
            let slot = slot.parse::<u8>().expect("Invalid slot");
            Some(Op::Store(slot))
        }

        "PRINT" => Some(Op::Print),

        "HALT" => Some(Op::Halt),

        _ => panic!("Unknown instruction: {}", instruction),
    }
}

pub fn assemble_file(input_path: &str, output_path: &str) {

    let source =
        fs::read_to_string(input_path).expect("Could not read the input file");

    let mut bytecode = Vec::new();
    let mut has_halt = false;

    for line in source.lines() {
        let result = parse_line(line);

        if let Some(op) = result {
            if op == Op::Halt {
                has_halt = true;
            }

            let encoded = op.encode();
            bytecode.extend(encoded);
        }
    }

    if !has_halt {
        eprintln!("Warning: Program does not contain HALT.");
    }

    let mut output = Vec::new();

    output.extend_from_slice(&[0x4D, 0x56, 0x4D, 0x00]);

    output.push(0x01);

    let length = bytecode.len() as u32;
    let length_bytes = length.to_le_bytes();
    output.extend_from_slice(&length_bytes);

    output.extend(bytecode);

    fs::write(output_path, output).expect("Could not write the output file");

    println!(
        "Successfully assembled {} to {}",
        input_path,
        output_path
    );
}