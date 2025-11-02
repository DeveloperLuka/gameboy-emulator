use super::*;

#[test]
fn test_dec_b_8_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x05;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.registers.b = 0x31;
    cpu.run();
    assert_eq!(cpu.registers.b, 0x30);
}

#[test]
fn test_dec_c_8_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x0D;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.registers.c = 0x41;
    cpu.run();
    assert_eq!(cpu.registers.c, 0x40);
}

#[test]
fn test_dec_d_8_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x15;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.registers.d = 0x26;
    cpu.run();
    assert_eq!(cpu.registers.d, 0x25);
}

#[test]
fn test_dec_e_8_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x1D;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.registers.e = 0x6A;
    cpu.run();
    assert_eq!(cpu.registers.e, 0x69);
}

#[test]
fn test_dec_h_8_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x25;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.registers.h = 0xAB;
    cpu.run();
    assert_eq!(cpu.registers.h, 0xAA);
}

#[test]
fn test_dec_l_8_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x2D;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.registers.l = 0xAF;
    cpu.run();
    assert_eq!(cpu.registers.l, 0xAE);
}

#[test]
fn test_dec_dereferenced_hl_8_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x35;
    memory.data[0x0400] = 0x55;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.registers.hl().set(0x0400);
    cpu.run();
    assert_eq!(memory.data[0x0400], 0x54);
}

#[test]
fn test_dec_a_8_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x3D;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.registers.a = 0x31;
    cpu.run();
    assert_eq!(cpu.registers.a, 0x30);
}
