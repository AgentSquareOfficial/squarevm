#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Push(i64), Pop, Dup, Swap, Add,
    Sub, Mul, Div, Mod, Neg, Load(u8),
    Store(u8), Print, Halt,
}

impl Op {
    pub fn encode(&self) -> Vec<u8> {
    let mut bytes = Vec::new();

    match self {
        Op::Push(n) => {
            bytes.push(0x01);
            bytes.extend_from_slice(&n.to_le_bytes()); // bytes = "[01]":opcode + "[2A][00][00][00]":operand
        }
        Op::Pop => bytes.push(0x02),
        Op::Dup => bytes.push(0x03),
        Op::Swap => bytes.push(0x04),
        Op::Add => bytes.push(0x10),
        Op::Sub => bytes.push(0x11),
        Op::Mul => bytes.push(0x12),
        Op::Div => bytes.push(0x13),
        Op::Mod => bytes.push(0x14),
        Op::Neg => bytes.push(0x15),
        Op::Load(slot) => {
            bytes.push(0x40);
            bytes.push(*slot);
        }
        Op::Store(slot) => {
            bytes.push(0x41);
            bytes.push(*slot);
        }
        Op::Print => bytes.push(0x60),
        Op::Halt => bytes.push(0xFF),
    }
        bytes
    }

    pub fn decode(bytes: &[u8]) -> Option<(Op, usize)> {
        if bytes.is_empty() { return None; }

        match bytes[0] {
            0x01 => {
                if bytes.len() < 9 { return None; } // Catch truncated instruction
                let mut buf = [0u8; 8];
                buf.copy_from_slice(&bytes[1..9]);
                let n = i64::from_le_bytes(buf);
                Some((Op::Push(n), 9))
            }
            0x02 => Some((Op::Pop, 1)),
            0x03 => Some((Op::Dup, 1)),
            0x04 => Some((Op::Swap, 1)),
            0x10 => Some((Op::Add, 1)),
            0x11 => Some((Op::Sub, 1)),
            0x12 => Some((Op::Mul, 1)),
            0x13 => Some((Op::Div, 1)),
            0x14 => Some((Op::Mod, 1)),
            0x15 => Some((Op::Neg, 1)),
            0x40 => {
                if bytes.len() < 2 { return None; }
                Some((Op::Load(bytes[1]), 2))
            }
            0x41 => {
                if bytes.len() < 2 { return None; }
                Some((Op::Store(bytes[1]), 2))
            }
            0x60 => Some((Op::Print, 1)),
            0xFF => Some((Op::Halt, 1)),
            _ => None, // Unknown opcode trap
        }
    }
}

