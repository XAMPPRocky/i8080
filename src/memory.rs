use std::ops::{Index, Range, RangeFrom};
use pointer::Pointer;

pub struct Memory {
    pub memory: Vec<u8>,
    pub ram_mirror: Option<u16>,
}

macro_rules! index_impl {
    (&$this:ident, $index:ident) => {
        match $this.ram_mirror {
            Some(mirror) if $index as u16 > mirror => {
                &$this.memory[((($index as u16) % mirror) + mirror) as usize]
            }
            _ => &$this.memory[$index as usize]
        }
    }
}

impl Memory {
    pub fn load(&mut self, block: &[u8], position: u16) {
        for (byte, pos) in block.iter().zip(position..) {
            self.memory[pos as usize] = *byte;
        }
    }

    pub fn write<A: Into<u16>, B: Into<u8>>(&mut self, address: A, value: B) {
        let address = address.into();
        let value = value.into();
        match self.ram_mirror {
            Some(mirror) if address >= mirror || address < 0x2000 => {},
            _ => self.memory[address as usize] = value,
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            memory: vec![0; 0x10000],
            ram_mirror: None,
        }
    }
}

impl Index<Range<usize>> for Memory {
    type Output = [u8];
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.memory[index]
    }
}

impl Index<RangeFrom<usize>> for Memory {
    type Output = [u8];
    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.memory[index]
    }
}

impl Index<Pointer> for Memory {
    type Output = u8;
    fn index(&self, index: Pointer) -> &Self::Output {
        &self[*index]
    }
}

impl Index<u8> for Memory {
    type Output = u8;
    fn index(&self, index: u8) -> &Self::Output {
        index_impl!(&self, index)
    }
}

impl Index<u16> for Memory {
    type Output = u8;
    fn index(&self, index: u16) -> &Self::Output {
        index_impl!(&self, index)
    }
}
