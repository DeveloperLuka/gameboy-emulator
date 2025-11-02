#[cfg(test)]
mod tests;

mod alu;
mod registers;

use crate::memory::Memory;

use registers::Registers;
use registers::U16Register;

pub struct CPU<'a> {
    registers: Registers,
    memory: &'a mut Memory,
}

impl<'a> CPU<'a> {
    pub fn new(memory: &'a mut Memory) -> Self {
        CPU {
            registers: Registers {
                a: 0,
                f: registers::Flag { f: 0 },
                b: 0,
                c: 0,
                d: 0,
                e: 0,
                h: 0,
                l: 0,
                sp: 0,
                pc: 0,
            },
            memory: memory,
        }
    }

    pub fn run(&mut self) {
        self.memory.data[0] = 0x06;
        self.memory.data[1] = 0x10;
        self.memory.data[2] = 0x04;
        self.memory.data[3] = 0x04;
        self.memory.data[4] = 0x04;
        self.memory.data[5] = 0x05;
        self.memory.data[6] = 0x01;
        self.memory.data[7] = 0x11;
        self.memory.data[8] = 0x11;
        self.memory.data[9] = 0x31;
        self.memory.data[10] = 0x12;
        self.memory.data[11] = 0x41;
        self.memory.data[12] = 0x09;
        self.memory.data[12] = 0x09;
        loop {
            let next_instruction = self.fetch_next_u8();
            self.execute_instruction(next_instruction);
            if (self.registers.pc as usize) >= 12 {
                break;
            }
        }

        let a = self.registers.a;
        println!("Register A: {a}");

        let b = self.registers.b;
        println!("Register B: {b}");

        let bc = self.registers.bc().get();
        println!("Register: bc {bc}");

        let sp = self.registers.sp().get();
        println!("Register: sp {sp}");

        let hl = self.registers.hl().get();
        println!("Register: hl {hl}");
    }

    fn fetch_next_u8(&mut self) -> u8 {
        let fetched_u8 = self.memory.data[self.registers.pc as usize];
        self.registers.pc += 1;
        return fetched_u8;
    }

    fn execute_instruction(&mut self, opcode: u8) {
        let first_digit = opcode >> 6;
        match first_digit {
            0o0 => self.execute_instruction_group_0(opcode),
            _ => unimplemented!(),
        }
    }

    fn execute_instruction_group_0(&mut self, opcode: u8) {
        let second_digit = (opcode >> 3)/* & 0o07*/; // TODO might need to change
        let third_digit = opcode & 0o007;
        match third_digit {
            0o1 => {
                if second_digit % 2 == 0 {
                    let valuehigh = self.fetch_next_u8();
                    let valuelow = self.fetch_next_u8();
                    ld_16_bit(
                        &mut self.registers.map_u16_register_from_octet(second_digit / 2),
                        (valuehigh as u16) << 8 | valuelow as u16,
                    );
                } else {
                    let value = self
                        .registers
                        .map_u16_register_from_octet((second_digit - 1) / 2)
                        .get();
                    let register_value = self.registers.hl().get();
                    let return_value =
                        alu::add_16_bit(&mut self.registers.f, register_value, value);
                    self.registers.hl().set(return_value);
                }
            }
            0o3 => {
                if second_digit % 2 == 0 {
                    inc_16_bit(&mut self.registers.map_u16_register_from_octet(second_digit / 2));
                } else {
                    dec_16_bit(
                        &mut self
                            .registers
                            .map_u16_register_from_octet((second_digit - 1) / 2),
                    );
                }
            }
            0o4 => {
                let value = *self.map_u8_from_octet(second_digit);
                *self.map_u8_from_octet(second_digit) =
                    alu::inc_8_bit(&mut self.registers.f, value);
            }
            0o5 => {
                let value = *self.map_u8_from_octet(second_digit);
                *self.map_u8_from_octet(second_digit) =
                    alu::dec_8_bit(&mut self.registers.f, value);
            }
            0o6 => {
                let value = self.fetch_next_u8();
                ld_8_bit(self.map_u8_from_octet(second_digit), value);
            }
            _ => unimplemented!(),
        }
    }

    pub fn map_u8_from_octet(&mut self, octet: u8) -> &mut u8 {
        match octet {
            0o0 => &mut self.registers.b,
            0o1 => &mut self.registers.c,
            0o2 => &mut self.registers.d,
            0o3 => &mut self.registers.e,
            0o4 => &mut self.registers.h,
            0o5 => &mut self.registers.l,
            0o6 => &mut self.memory.data[self.registers.hl().get() as usize],
            0o7 => &mut self.registers.a,
            _ => unimplemented!(),
        }
    }
}

fn ld_8_bit(register: &mut u8, value: u8) {
    *register = value;
}

fn inc_16_bit(register: &mut U16Register) {
    register.set(register.get() + 1);
}

fn dec_16_bit(register: &mut U16Register) {
    register.set(register.get() - 1);
}

fn ld_16_bit(register: &mut U16Register, value: u16) {
    register.set(value);
}
