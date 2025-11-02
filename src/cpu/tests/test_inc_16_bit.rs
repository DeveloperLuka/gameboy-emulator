use super::*;

#[test]
fn test_inc_bc_16_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x03;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.registers.bc().set(0x4923);
    cpu.run();
    assert_eq!(cpu.registers.bc().get(), 0x4924);
}

#[test]
fn test_inc_de_16_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x13;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.registers.de().set(0x4212);
    cpu.run();
    assert_eq!(cpu.registers.de().get(), 0x4213);
}

#[test]
fn test_inc_hl_16_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x23;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.registers.hl().set(0x3E4A);
    cpu.run();
    assert_eq!(cpu.registers.hl().get(), 0x3E4B);
}

#[test]
fn test_inc_sp_16_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x33;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.registers.sp().set(0x1214);
    cpu.run();
    assert_eq!(cpu.registers.sp().get(), 0x1215);
}
