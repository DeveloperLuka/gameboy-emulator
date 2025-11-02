use super::*;

#[test]
fn test_inc_b_8_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x04;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.registers.b = 0x31;
    cpu.run();
    assert_eq!(cpu.registers.b, 0x32);
}

#[test]
fn test_inc_c_8_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x0C;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.registers.c = 0x41;
    cpu.run();
    assert_eq!(cpu.registers.c, 0x42);
}

#[test]
fn test_inc_d_8_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x14;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.registers.d = 0x26;
    cpu.run();
    assert_eq!(cpu.registers.d, 0x27);
}

#[test]
fn test_inc_e_8_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x1C;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.registers.e = 0x6A;
    cpu.run();
    assert_eq!(cpu.registers.e, 0x6B);
}

#[test]
fn test_inc_h_8_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x24;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.registers.h = 0xAB;
    cpu.run();
    assert_eq!(cpu.registers.h, 0xAC);
}

#[test]
fn test_inc_l_8_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x2C;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.registers.l = 0xAF;
    cpu.run();
    assert_eq!(cpu.registers.l, 0xB0);
}

#[test]
fn test_inc_dereferenced_hl_8_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x34;
    memory.data[0x0400] = 0x55;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.registers.hl().set(0x0400);
    cpu.run();
    assert_eq!(memory.data[0x0400], 0x56);
}

#[test]
fn test_inc_a_8_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x3C;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.registers.a = 0x31;
    cpu.run();
    assert_eq!(cpu.registers.a, 0x32);
}
