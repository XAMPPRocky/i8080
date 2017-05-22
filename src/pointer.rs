#[derive(Clone, Copy)]
pub struct Pointer(pub u16);

impl Default for Pointer {
    fn default() -> Self {
        Pointer(0)
    }
}

impl From<u16> for Pointer {
    fn from(from: u16) -> Self {
        Pointer(from)
    }
}

impl From<u8> for Pointer {
    fn from(from: u8) -> Self {
        Pointer(from as u16)
    }
}

impl From<Pointer> for u16 {
    fn from(from: Pointer) -> Self {
       from.0
    }
}

impl From<Pointer> for u8 {
    fn from(from: Pointer) -> Self {
        from.0 as u8
    }
}
