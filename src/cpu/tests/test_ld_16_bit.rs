use super::*;

#[test]
fn test_ld_bc_16_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x01;
    memory.data[0x0101] = 0x11;
    memory.data[0x0102] = 0x41;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.run();
    assert_eq!(cpu.registers.bc().get(), 0x1141);
}

#[test]
fn test_ld_de_16_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x11;
    memory.data[0x0101] = 0x54;
    memory.data[0x0102] = 0x42;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.run();
    assert_eq!(cpu.registers.de().get(), 0x5442);
}

#[test]
fn test_ld_hl_16_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x21;
    memory.data[0x0101] = 0x24;
    memory.data[0x0102] = 0x22;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.run();
    assert_eq!(cpu.registers.hl().get(), 0x2422);
}

#[test]
fn test_ld_sp_16_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x31;
    memory.data[0x0101] = 0x14;
    memory.data[0x0102] = 0x26;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.run();
    assert_eq!(cpu.registers.sp().get(), 0x1426);
}
