pub mod cpu;

#[test]
fn works() {
    assert_eq!(2 + 2, 4);
}

fn main() {
    let mut cpu = cpu::CPU::new();
    cpu.run();
    println!("Hello, world!");
}
