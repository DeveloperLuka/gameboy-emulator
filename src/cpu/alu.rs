use super::registers::Flag;

pub fn add_16_bit(flags: &mut Flag, value1: u16, value2: u16) -> u16 {
    let (new_value, overflow) = value1.overflowing_add(value2);
    flags.set_substraction(false);
    flags.set_half_carry(((value1 & 0x0FFF) + (value2 & 0x0FFF)) > 0x0FFF);
    flags.set_carry(overflow);
    new_value
}

pub fn inc_8_bit(flags: &mut Flag, value: u8) -> u8 {
    let new_value = value.wrapping_add(1);
    flags.set_zero(new_value == 0);
    flags.set_substraction(false);
    flags.set_half_carry((value & 0x0F) + 1 > 0x0F);
    new_value
}

pub fn dec_8_bit(flags: &mut Flag, value: u8) -> u8 {
    let new_value = value.wrapping_sub(1);
    flags.set_zero(new_value == 0);
    flags.set_substraction(true);
    flags.set_half_carry((value & 0x0F) == 0);
    new_value
}
