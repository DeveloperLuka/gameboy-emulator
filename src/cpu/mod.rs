mod registers;

use registers::Registers;
use registers::U16Register;

pub struct CPU {
    registers: Registers,
    instructions: Box<[u8]>,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: Registers {
                a: 0,
                f: 0,
                b: 0,
                c: 0,
                d: 0,
                e: 0,
                h: 0,
                l: 0,
                sp: 0,
                pc: 0,
            },
            instructions: Box::new([
                0x06, 0x10, 0x04, 0x04, 0x04, 0x05, 0x01, 0x11, 0x11, 0x31, 0x12, 0x41, 0x09, 0x09,
            ]),
        }
    }

    pub fn run(&mut self) {
        loop {
            let next_instruction = self.fetch_next_u8();
            self.execute_instruction(next_instruction);
            if (self.registers.pc as usize) >= self.instructions.len() {
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
        let fetched_u8 = self.instructions[self.registers.pc as usize];
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
                    add_16_bit(&mut self.registers.hl(), value);
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
                inc_8_bit(self.registers.map_register_from_octet(second_digit));
            }
            0o5 => {
                dec_8_bit(self.registers.map_register_from_octet(second_digit));
            }
            0o6 => {
                let value = self.fetch_next_u8();
                ld_8_bit(self.registers.map_register_from_octet(second_digit), value);
            }
            _ => unimplemented!(),
        }
    }
}

fn inc_8_bit(register: &mut u8) {
    *register += 1;
}

fn dec_8_bit(register: &mut u8) {
    *register -= 1;
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

fn add_16_bit(register: &mut U16Register, value: u16) {
    register.set(register.get() + value);
}
