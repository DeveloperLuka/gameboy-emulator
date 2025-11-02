pub mod cpu;
mod memory;

fn main() {
    let mut memory = memory::Memory::new();
    let mut cpu = cpu::CPU::new(&mut memory);
    cpu.run();
    println!("Hello, world!");
}
