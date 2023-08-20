use crate::opcodes::{ArithmeticTarget, Opcode};

struct CPU {
    registers: Registers,
    sp: u16, // stack pointer
    pc: u16, // program counter
    memory_bus: MemoryBus,
}

impl CPU {
    fn execute(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::NOP => {}
            Opcode::ADD(target) => match target {
                ArithmeticTarget::A => {}
                ArithmeticTarget::B => {}
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::D => {}
                ArithmeticTarget::E => {}
                ArithmeticTarget::H => {}
                ArithmeticTarget::L => {}
            },
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.carry = did_overflow;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }
}

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8,
}

impl Registers {
    fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister::new(),
            h: 0,
            l: 0,
        }
    }

    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }
}

struct FlagsRegister {
    zero: bool, // set to true if the result of the operation is equal to 0. Used by conditional jumps
    subtract: bool, // set to true if the operation was a subtraction
    half_carry: bool, // set to true if there is an overflow from the lower nibble (a.k.a the lower four bits) to the upper nibble (a.k.a the upper four bits)
    carry: bool,      // set to true if the operation resulted in an overflow
}

impl FlagsRegister {
    fn new() -> Self {
        Self {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false,
        }
    }
}

struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}
