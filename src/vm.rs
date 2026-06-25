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

    pub fn push(&mut self, val: i64, ip: usize) {
        if self.stack.len() >= 1024 {
            panic!("trap at ip={:#04X}: stack overflow", ip);
        }
        self.stack.push(val);
    }

    pub fn pop(&mut self, ip: usize) -> i64 {
        self.stack.pop().unwrap_or_else(|| {
            panic!("trap at ip={:#04X}: stack underflow", ip)
        })
    }

    pub fn execute(&mut self, op: Op, ip: usize) {
        match op {
            Op::Push(n) => self.push(n, ip),
            Op::Pop => { 
                self.pop(ip); // Pops and throws away the value
            }
            Op::Dup => {
                let val = *self.stack.last().unwrap_or_else(|| {
                    panic!("trap at ip={:#04X}: stack underflow (DUP on empty stack)", ip)
                });
                self.push(val, ip);
            }
            Op::Swap => {
                let len = self.stack.len();
                if len < 2 {
                    panic!("trap at ip={:#04X}: stack underflow (SWAP requires 2 elements)", ip);
                }
                self.stack.swap(len - 1, len - 2);
            }
            
            // --- Arithmetic Operations ---
            
            Op::Add => {
                let b = self.pop(ip);
                let a = self.pop(ip);
                self.push(a + b, ip);
            }
            Op::Sub => {
                let b = self.pop(ip);
                let a = self.pop(ip);
                self.push(a - b, ip);
            }
            Op::Mul => {
                let b = self.pop(ip);
                let a = self.pop(ip);
                self.push(a * b, ip);
            }
            Op::Div => {
                let b = self.pop(ip);
                let a = self.pop(ip);
                if b == 0 {
                    panic!("trap at ip={:#04X}: division by zero", ip);
                }
                self.push(a / b, ip);
            }
            Op::Mod => {
                let b = self.pop(ip);
                let a = self.pop(ip);
                if b == 0 {
                    panic!("trap at ip={:#04X}: modulo by zero", ip);
                }
                self.push(a % b, ip);
            }
            Op::Neg => {
                let a = self.pop(ip);
                self.push(-a, ip);
            }
            
            // --- Memory and I/O ---
            
            Op::Load(slot) => {
                let val = self.globals[slot as usize];
                self.push(val, ip);
            }
            Op::Store(slot) => {
                let val = self.pop(ip);
                self.globals[slot as usize] = val;
            }
            Op::Print => {
                let val = self.pop(ip);
                println!("{}", val);
            }
            Op::Halt => {
                self.ip = self.code.len(); 
            }
        }
    }
}