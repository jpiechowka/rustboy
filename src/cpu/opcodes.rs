// https://gbdev.io/gb-opcodes/optables
// https://gbdev.io/gb-opcodes/Opcodes.json

pub enum Opcode {
    NOP,
    ADD(ArithmeticTarget),
}

pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl Opcode {
    pub fn from_byte(byte: u8, is_prefixed: bool) -> Option<Opcode> {
        if is_prefixed {
            return Opcode::from_byte_prefixed(byte);
        }
        Opcode::from_byte_not_prefixed(byte)
    }

    fn from_byte_prefixed(byte: u8) -> Option<Opcode> {
        match byte {
            _ => None,
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Opcode> {
        match byte {
            0x00 => Some(Opcode::NOP),
            _ => None,
        }
    }
}
