#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Register(pub u8);

impl Register {
    pub fn to_u16(self) -> u16 {
        self.0 as u16
    }
}

impl From<u16> for Register {
    fn from(from: u16) -> Self {
        Register(from as u8)
    }
}

impl From<u8> for Register {
    fn from(from: u8) -> Self {
        Register(from)
    }
}

impl From<Register> for u8 {
    fn from(from: Register) -> Self {
        from.0
    }
}
