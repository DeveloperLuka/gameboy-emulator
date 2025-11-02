pub struct Registers {
    pub a: u8,
    pub f: Flag,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    pub fn bc(&mut self) -> U16Register<'_> {
        U16Register {
            value: U16Value::U8 {
                low: &mut self.b,
                high: &mut self.c,
            },
        }
    }

    pub fn de(&mut self) -> U16Register<'_> {
        U16Register {
            value: U16Value::U8 {
                low: &mut self.d,
                high: &mut self.e,
            },
        }
    }

    pub fn hl(&mut self) -> U16Register<'_> {
        U16Register {
            value: U16Value::U8 {
                low: &mut self.h,
                high: &mut self.l,
            },
        }
    }

    pub fn sp(&mut self) -> U16Register<'_> {
        U16Register {
            value: U16Value::U16 {
                value: &mut self.sp,
            },
        }
    }

    pub fn map_u16_register_from_octet(&mut self, octet: u8) -> U16Register<'_> {
        match octet {
            0o0 => self.bc(),
            0o1 => self.de(),
            0o2 => self.hl(),
            0o3 => self.sp(),
            _ => unimplemented!(),
        }
    }
}

enum U16Value<'a> {
    U8 { low: &'a mut u8, high: &'a mut u8 },
    U16 { value: &'a mut u16 },
}

pub struct U16Register<'a> {
    value: U16Value<'a>,
}

impl<'a> U16Register<'a> {
    pub fn get(&self) -> u16 {
        match &self.value {
            U16Value::U8 { low, high } => (**high as u16) << 8 | **low as u16,
            U16Value::U16 { value } => **value,
        }
    }

    pub fn set(&mut self, new_value: u16) {
        match &mut self.value {
            U16Value::U8 { low, high } => {
                **high = (new_value >> 8) as u8;
                **low = (new_value & 0x00FF) as u8;
            }
            U16Value::U16 { value } => **value = new_value,
        }
    }
}

const Z_FLAG: u8 = 0b1000_0000;
const N_FLAG: u8 = 0b0100_0000;
const H_FLAG: u8 = 0b0010_0000;
const C_FLAG: u8 = 0b0001_0000;

pub struct Flag {
    pub f: u8,
}

impl Flag {
    fn set_flag(&mut self, mask: u8, state: bool) {
        if state {
            self.f |= mask;
        } else {
            self.f &= !mask;
        }
    }

    pub fn is_zero(&self) -> bool {
        self.f & Z_FLAG != 0
    }

    pub fn set_zero(&mut self, zero: bool) {
        self.set_flag(Z_FLAG, zero);
    }

    pub fn is_substraction(&self) -> bool {
        self.f & N_FLAG != 0
    }

    pub fn set_substraction(&mut self, substraction: bool) {
        self.set_flag(N_FLAG, substraction);
    }

    pub fn is_half_carry(&self) -> bool {
        self.f & H_FLAG != 0
    }

    pub fn set_half_carry(&mut self, half_carry: bool) {
        self.set_flag(H_FLAG, half_carry);
    }

    pub fn is_carry(&self) -> bool {
        self.f & C_FLAG != 0
    }

    pub fn set_carry(&mut self, carry: bool) {
        self.set_flag(C_FLAG, carry);
    }
}
