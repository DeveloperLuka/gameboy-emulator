pub struct Registers {
    pub a: u8,
    pub f: u8,
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

    pub fn map_register_from_octet(&mut self, octet: u8) -> &mut u8 {
        match octet {
            0o0 => &mut self.b,
            0o1 => &mut self.c,
            0o2 => &mut self.d,
            0o3 => &mut self.e,
            0o4 => &mut self.h,
            0o5 => &mut self.l,
            0o6 => &mut self.b, // TODO wherever hl is pointing to has to be hanlded extra
            0o7 => &mut self.a,
            _ => unimplemented!(),
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
