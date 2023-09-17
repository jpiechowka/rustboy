use log::{error, warn};

use opcodes::{ArithmeticTarget, Opcode};
use registers::Registers;

use crate::memory_bus::MemoryBus;

// TODO: Check visibility
mod flags_register;
mod opcodes;
// TODO: Decide which to use
pub mod opcodes_json;
// TODO: Decide which to use
mod registers;

struct CPU {
    registers: Registers,
    sp: u16,
    // stack pointer
    pc: u16,
    // program counter
    memory_bus: MemoryBus,
}

impl CPU {
    fn step(&mut self) {
        let mut opcode_byte = self.memory_bus.read_byte(self.pc);

        let is_prefixed = opcode_byte == 0xCB;
        if is_prefixed {
            opcode_byte = self.memory_bus.read_byte(self.pc + 1);
        }

        let next_program_counter: u16 =
            if let Some(opcode) = Opcode::from_byte(opcode_byte, is_prefixed) {
                self.execute(opcode)
            } else {
                // TODO: panic instead of logging error?
                let msg = format!("0x{}{:x}", if is_prefixed { "CB" } else { "" }, opcode_byte);
                error!("Encountered unknown opcode for: {}", msg);
                warn!(
                    "Incrementing program counter (current val: {}) by 1 and proceeding",
                    self.pc
                );
                self.pc.wrapping_add(1)
            };

        self.pc = next_program_counter;
    }

    fn execute(&mut self, opcode: Opcode) -> u16 {
        match opcode {
            Opcode::NOP => self.pc.wrapping_add(1),
            Opcode::ADD(target) => match target {
                ArithmeticTarget::A => self.pc.wrapping_add(1), // TODO: implement
                ArithmeticTarget::B => self.pc.wrapping_add(1), // TODO: implement
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::D => self.pc.wrapping_add(1), // TODO: implement
                ArithmeticTarget::E => self.pc.wrapping_add(1), // TODO: implement
                ArithmeticTarget::H => self.pc.wrapping_add(1), // TODO: implement
                ArithmeticTarget::L => self.pc.wrapping_add(1), // TODO: implement
            },
        }
    }

    // TODO: implement push_stack and memory_bus.write_byte
    // fn push_stack(&mut self, value: u16) -> u16 {
    //     self.sp = self.sp.wrapping_sub(1);
    //     self.memory_bus
    //         .write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);

    //     self.sp = self.sp.wrapping_sub(1);
    //     self.memory_bus.write_byte(self.sp, (value & 0xFF) as u8);
    // }

    fn pop_stack(&mut self) -> u16 {
        let lsb = self.memory_bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.memory_bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (msb << 8) | lsb
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
