use super::*;

#[test]
fn test_ld_b_8_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x06;
    memory.data[0x0101] = 0x30;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.run();
    assert_eq!(cpu.registers.b, 0x30);
}

#[test]
fn test_ld_c_8_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x0E;
    memory.data[0x0101] = 0x40;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.run();
    assert_eq!(cpu.registers.c, 0x40);
}

#[test]
fn test_ld_d_8_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x16;
    memory.data[0x0101] = 0x45;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.run();
    assert_eq!(cpu.registers.d, 0x45);
}

#[test]
fn test_ld_e_8_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x1E;
    memory.data[0x0101] = 0x46;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.run();
    assert_eq!(cpu.registers.e, 0x46);
}

#[test]
fn test_ld_h_8_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x26;
    memory.data[0x0101] = 0x31;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.run();
    assert_eq!(cpu.registers.h, 0x31);
}

#[test]
fn test_ld_l_8_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x2E;
    memory.data[0x0101] = 0x53;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.run();
    assert_eq!(cpu.registers.l, 0x53);
}

#[test]
fn test_ld_dereferenced_hl_8_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x36;
    memory.data[0x0101] = 0x39;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.registers.hl().set(0x0400);
    cpu.run();
    assert_eq!(memory.data[0x0400], 0x39);
}

#[test]
fn test_ld_a_8_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x3E;
    memory.data[0x0101] = 0x03;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.run();
    assert_eq!(cpu.registers.a, 0x03);
}
