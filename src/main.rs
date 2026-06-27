use std::env;

pub mod asm;
pub mod dis;
pub mod isa;
pub mod vm;

fn main() {
    // Grab the command line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage:");
        eprintln!("  minivm asm <file.tasm> -o <file.tbc>");
        eprintln!("  minivm dis <file.tbc>");
        eprintln!("  minivm run <file.tbc>");
        std::process::exit(1);
    }

    let command = &args[1];
    let file = &args[2];

    match command.as_str() {
        "asm" => {
            if args.len() == 5 && args[3] == "-o" {
                asm::assemble_file(file, &args[4]);
            } else {
                eprintln!("Usage: minivm asm <file.tasm> -o <file.tbc>");
            }
        }
        "dis" => {
            dis::disassemble_file(file);
        }
        "run" => {
            let bytes = std::fs::read(file).expect("Failed to read bytecode file");
            
            let code_only = bytes[9..].to_vec();
            
            let mut machine = vm::Vm::new(code_only);
            machine.run();
        }
        _ => eprintln!("Unknown command: {}", command),
    }
}