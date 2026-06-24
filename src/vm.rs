use crate::isa::Op;

pub struct Vm {
    code: Vec<u8>,  // The frozen bytecode program
    ip: usize, // Instruction Pointer
    stack: Vec<i64>,  // Operand stack
    globals: [i64; 256],  // 256 global slots
}

impl Vm {
    pub fn new(code: Vec<u8>) -> Self {
        Self {
            code: code,
            ip: 0,
            stack: Vec::with_capacity(1024),
            globals: [0; 256],
        }
    }

    // loop the ip(instructor pointer)
    pub fn run(&mut self) {
        while self.ip < self.code.len() {
            // Fetch the remaining bytes starting from the current IP
            let current_bytes = &self.code[self.ip..];

            // Decode the bytes into an Op enum
            // If it returns None, it means we hit an unknown opcode or truncated instruction
            let (op, bytes_consumed) = match Op::decode(current_bytes) {
                Some(result) => result,
                None => panic!("trap at ip={:#04X}: unknown opcode or truncated instruction", self.ip),
            };
            
            // Move the Instruction Pointer forward BEFORE executing
            // so if we trap inside execute, the IP points to the offending instruction
            let offending_ip = self.ip;
            self.ip += bytes_consumed;

            // Execute the instruction
            self.execute(op, offending_ip);
        }
    }
}