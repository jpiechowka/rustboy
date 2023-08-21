use crate::memory_bus::MemoryBus;
use crate::opcodes::{ArithmeticTarget, Opcode};
use crate::registers::Registers;

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
