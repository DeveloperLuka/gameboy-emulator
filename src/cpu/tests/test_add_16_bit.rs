use super::*;

#[test]
fn test_add_hl_bc_16_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x09;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.registers.hl().set(0x312);
    cpu.registers.bc().set(0x5121);
    cpu.run();
    assert_eq!(cpu.registers.hl().get(), 0x312 + 0x5121);
}

#[test]
fn test_add_hl_de_16_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x19;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.registers.hl().set(0x312);
    cpu.registers.de().set(0x521);
    cpu.run();
    assert_eq!(cpu.registers.hl().get(), 0x312 + 0x521);
}

#[test]
fn test_add_hl_hl_16_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x29;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.registers.hl().set(0x312);
    cpu.run();
    assert_eq!(cpu.registers.hl().get(), 0x312 + 0x0312);
}

#[test]
fn test_add_hl_sp_16_bit() {
    let mut memory = Memory::new();
    memory.data[0x0100] = 0x39;
    let mut cpu = CPU::new(&mut memory);
    cpu.registers.pc = 0x0100;
    cpu.registers.hl().set(0x312);
    cpu.registers.sp().set(0x19F3);
    cpu.run();
    assert_eq!(cpu.registers.hl().get(), 0x312 + 0x19F3);
}
