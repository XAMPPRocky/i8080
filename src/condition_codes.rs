use std::fmt;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct ConditionCodes {
    pub z: bool,
    pub s: bool,
    pub p: bool,
    pub cy: bool,
    pub ac: bool,
}

impl ConditionCodes {
    pub fn set_z(&mut self, answer: u16) {
        self.z = answer & 0xff == 0;
    }

    pub fn set_s(&mut self, answer: u16) {
        self.s = answer & 0x80 != 0;
    }

    pub fn set_cy(&mut self, answer: u16) {
        self.cy = answer > 0xff;
    }

    pub fn set_p(&mut self, answer: u16) {
        self.p = (answer as u8).count_ones() % 2 == 0;
    }

    pub fn set_ac(&mut self, answer: u8) {
        self.ac = answer > 0xf;
    }

    pub fn set_all(&mut self, answer: u16, ac_check: u8) {
        self.set_all_except_carry(answer, ac_check);
        self.set_cy(answer);
    }

    pub fn set_all_except_carry(&mut self, answer: u16, ac_check: u8) {
        self.set_z(answer);
        self.set_s(answer);
        self.set_p(answer);
        self.set_ac(ac_check);
    }

    pub fn set_all_except_ac(&mut self, answer: u16) {
        self.set_z(answer);
        self.set_s(answer);
        self.set_p(answer);
        self.set_cy(answer);
    }
}

// FLAG WORD:
// -----------------------------------
// | S | Z | 0 | AC | 0 | P | 1 | CY |
// -----------------------------------
impl From<ConditionCodes> for u8 {
    fn from(from: ConditionCodes) -> Self {
        let mut word = 0;

        word |= from.s as u8;
        word <<= 1;

        word |= from.z as u8;
        word <<= 2;

        word |= from.ac as u8;
        word <<= 2;

        word |= from.p as u8;
        word <<= 1;

        word |= 1;
        word <<= 1;

        word |= from.cy as u8;

        word
    }
}

impl From<u8> for ConditionCodes {
    fn from(from: u8) -> Self {
        let mut conditions = ConditionCodes::default();

        conditions.cy = from & 0x1 != 0;
        conditions.p = from & 0x4 != 0;
        conditions.ac = from & 0x10 != 0;
        conditions.z = from & 0x40 != 0;
        conditions.s = from & 0x80 != 0;

        conditions
    }
}

impl fmt::Debug for ConditionCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        macro_rules! dot {
            ($name:ident) => {
                write!(f, "{}", if self.$name {
                    stringify!($name)
                } else {
                    "."
                })
            }
        }

        dot!(s)?;
        dot!(z)?;
        dot!(ac)?;
        dot!(p)?;
        dot!(cy)
    }
}
