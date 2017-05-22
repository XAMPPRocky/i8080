use std::ops::{Index, IndexMut, Range, RangeFrom};
use pointer::Pointer;

pub struct Memory {
    pub memory: Vec<u8>,
    pub ram_mirror: Option<u16>,

}

macro_rules! index_impl {
    (&mut$this:ident, $index:ident) => {
        match $this.ram_mirror {
            Some(mirror) if $index as u16 > mirror => {
                &mut $this.memory[((($index as u16) % mirror) + mirror) as usize]
            }
            _ => &mut $this.memory[$index as usize]
        }
    };

    (&$this:ident, $index:ident) => {
        match $this.ram_mirror {
            Some(mirror) if $index as u16 > mirror => {
                &$this.memory[((($index as u16) % mirror) + mirror) as usize]
            }
            _ => &$this.memory[$index as usize]
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

impl IndexMut<Pointer> for Memory {
    fn index_mut(&mut self, index: Pointer) -> &mut Self::Output {
        &mut self[*index]
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

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        index_impl!(&mut self, index)
    }
}
