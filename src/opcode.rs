use std::fmt;
use std::ops::{Deref, DerefMut};

const CYCLES: [u8; 256] = [
    4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5, 7, 4, //0x00..0x0f
    4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5, 7, 4, //0x10..0x1f
    4, 10, 16, 5, 5, 5, 7, 4, 4, 10, 16, 5, 5, 5, 7, 4, //etc
    4, 10, 13, 5, 10, 10, 10, 4, 4, 10, 13, 5, 5, 5, 7, 4,

    5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5, //0x40..0x4f
    5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5,
    5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5,
    7, 7, 7, 7, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 7, 5,

    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4, //0x80..8x4f
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,

    11, 10, 10, 10, 17, 11, 7, 11, 11, 10, 10, 10, 10, 17, 7, 11, //0xc0..0xcf
    11, 10, 10, 10, 17, 11, 7, 11, 11, 10, 10, 10, 10, 17, 7, 11,
    11, 10, 10, 18, 17, 11, 7, 11, 11, 5, 10, 5, 17, 17, 7, 11,
    11, 10, 10, 4, 17, 11, 7, 11, 11, 5, 10, 4, 17, 17, 7, 11,
];

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Opcode(u8);

impl Opcode {
    pub fn size(&self) -> u8 {
        match self.0 {
            0x01 | 0x11 | 0x21 | 0x22 | 0x2a | 0x31 | 0x32 | 0x3a | 0xc2 |
            0xc3 | 0xc4 | 0xCA | 0xCC | 0xCD | 0xD2 | 0xD4 | 0xDA | 0xDC |
            0xE2 | 0xE4 | 0xEA | 0xEC | 0xF2 | 0xF4 | 0xFA | 0xFC | 0xDD |
            0xED | 0xFD => 3,

            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E | 0xC6 |
            0xCE | 0xD3 | 0xDE | 0xD6 | 0xDB | 0xe6 | 0xEE | 0xF6 | 0xFE => 2,

            _ => 1,
        }
    }

    pub fn cycle_size(&self) -> u8 {

        if self.0 == 0xd3 || self.0 == 0xdb {
            3
        } else {
            CYCLES[self.0 as usize]
        }
    }
}

impl From<u8> for Opcode {
    fn from(new: u8) -> Self {
        Opcode(new)
    }
}

impl Deref for Opcode {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Opcode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let op = match self.0 {
            0x00 => "NOP",
            0x01 => "LXI B",
            0x02 => "STAX B",
            0x03 => "INX B",
            0x04 => "INR B",
            0x05 => "DCR B",
            0x06 => "MVI B",
            0x07 => "RLC",
            0x08 => "0x08",
            0x09 => "DAD B",
            0x0A => "LDAX B",
            0x0B => "DCX B",
            0x0C => "INR C",
            0x0D => "DCR C",
            0x0E => "MVI C",
            0x0F => "RRC",
            0x10 => "0x10",
            0x11 => "LXI D",
            0x12 => "STAX D",
            0x13 => "INX D",
            0x14 => "INR D",
            0x15 => "DCR D",
            0x16 => "MVI D",
            0x17 => "RAL",
            0x18 => "0x18",
            0x19 => "DAD D",
            0x1a => "LDAX D",
            0x1b => "DCX D",
            0x1c => "INR E",
            0x1d => "DCR E",
            0x1e => "MVI E",
            0x1f => "RAR",
            0x20 => "0x20",
            0x21 => "LXI H",
            0x22 => "SHLD",
            0x23 => "INX H",
            0x24 => "INR H",
            0x25 => "DCR H",
            0x26 => "MVI H",
            0x27 => "DAA",
            0x28 => "0x28",
            0x29 => "DAD H",
            0x2a => "LHLD",
            0x2b => "DCX H",
            0x2c => "INR L",
            0x2d => "DCR L",
            0x2e => "MVI L",
            0x2f => "CMA",
            0x30 => "0x30",
            0x31 => "LXI SP",
            0x32 => "STA",
            0x33 => "INX SP",
            0x34 => "INR M",
            0x35 => "DCR M",
            0x36 => "MVI M",
            0x37 => "STC",
            0x38 => "0x38",
            0x39 => "DAD SP",
            0x3a => "LDA",
            0x3b => "DCX SP",
            0x3c => "INR A",
            0x3d => "DCR A",
            0x3e => "MVI A",
            0x3f => "CMC",
            0x40 => "MOV B, B",
            0x41 => "MOV B, C",
            0x42 => "MOV B, D",
            0x43 => "MOV B, E",
            0x44 => "MOV B, H",
            0x45 => "MOV B, L",
            0x46 => "MOV B, M",
            0x47 => "MOV B, A",
            0x48 => "MOV C, B",
            0x49 => "MOV C, C",
            0x4a => "MOV C, D",
            0x4b => "MOV C, E",
            0x4c => "MOV C, H",
            0x4d => "MOV C, L",
            0x4e => "MOV C, M",
            0x4f => "MOV C, A",
            0x50 => "MOV D, B",
            0x51 => "MOV D, C",
            0x52 => "MOV D, D",
            0x53 => "MOV D, E",
            0x54 => "MOV D, H",
            0x55 => "MOV D, L",
            0x56 => "MOV D, M",
            0x57 => "MOV D, A",
            0x58 => "MOV E, B",
            0x59 => "MOV E, C",
            0x5a => "MOV E, D",
            0x5b => "MOV E, E",
            0x5c => "MOV E, H",
            0x5d => "MOV E, L",
            0x5e => "MOV E, M",
            0x5f => "MOV E, A",
            0x60 => "MOV H, B",
            0x61 => "MOV H, C",
            0x62 => "MOV H, D",
            0x63 => "MOV H, E",
            0x64 => "MOV H, H",
            0x65 => "MOV H, L",
            0x66 => "MOV H, M",
            0x67 => "MOV H, A",
            0x68 => "MOV L, B",
            0x69 => "MOV L, C",
            0x6a => "MOV L, D",
            0x6b => "MOV L, E",
            0x6c => "MOV L, H",
            0x6d => "MOV L, L",
            0x6e => "MOV L, M",
            0x6f => "MOV L, A",
            0x70 => "MOV M, B",
            0x71 => "MOV M, C",
            0x72 => "MOV M, D",
            0x73 => "MOV M, E",
            0x74 => "MOV M, H",
            0x75 => "MOV M, L",
            0x76 => "HLT",
            0x77 => "MOV M, A",
            0x78 => "MOV A, B",
            0x79 => "MOV A, C",
            0x7a => "MOV A, D",
            0x7b => "MOV A, E",
            0x7c => "MOV A, H",
            0x7d => "MOV A, L",
            0x7e => "MOV A, M",
            0x7f => "MOV A, A",
            0x80 => "ADD B",
            0x81 => "ADD C",
            0x82 => "ADD D",
            0x83 => "ADD E",
            0x84 => "ADD H",
            0x85 => "ADD L",
            0x86 => "ADD M",
            0x87 => "ADD A",
            0x88 => "ADC B",
            0x89 => "ADC C",
            0x8a => "ADC D",
            0x8b => "ADC E",
            0x8c => "ADC H",
            0x8d => "ADC L",
            0x8e => "ADC M",
            0x8f => "ADC A",
            0x90 => "SUB B",
            0x91 => "SUB C",
            0x92 => "SUB D",
            0x93 => "SUB E",
            0x94 => "SUB H",
            0x95 => "SUB L",
            0x96 => "SUB M",
            0x97 => "SUB A",
            0x98 => "SBB B",
            0x99 => "SBB C",
            0x9a => "SBB D",
            0x9b => "SBB E",
            0x9c => "SBB H",
            0x9d => "SBB L",
            0x9e => "SBB M",
            0x9f => "SBB A",
            0xa0 => "ANA B",
            0xa1 => "ANA C",
            0xa2 => "ANA D",
            0xa3 => "ANA E",
            0xa4 => "ANA H",
            0xa5 => "ANA L",
            0xa6 => "ANA M",
            0xa7 => "ANA A",
            0xa8 => "XRA B",
            0xa9 => "XRA C",
            0xaa => "XRA D",
            0xab => "XRA E",
            0xac => "XRA H",
            0xad => "XRA L",
            0xae => "XRA M",
            0xaf => "XRA A",
            0xb0 => "ORA B",
            0xb1 => "ORA C",
            0xb2 => "ORA D",
            0xb3 => "ORA E",
            0xb4 => "ORA H",
            0xb5 => "ORA L",
            0xb6 => "ORA M",
            0xb7 => "ORA A",
            0xb8 => "CMP B",
            0xb9 => "CMP C",
            0xba => "CMP D",
            0xbb => "CMP E",
            0xbc => "CMP H",
            0xbd => "CMP L",
            0xbe => "CMP M",
            0xbf => "CMP A",
            0xc0 => "RNZ",
            0xc1 => "POP B",
            0xc2 => "JNZ",
            0xc3 => "JMP",
            0xc4 => "CNZ",
            0xc5 => "PUSH B",
            0xc6 => "ADI",
            0xc7 => "RST 0",
            0xc8 => "RZ",
            0xc9 => "RET",
            0xca => "JZ",
            0xcb => "0xcb",
            0xcc => "CZ",
            0xcd => "CALL",
            0xce => "ACI",
            0xcf => "RST 1",
            0xd0 => "RNC",
            0xd1 => "POP D",
            0xd2 => "JNC",
            0xd3 => "OUT",
            0xd4 => "CNC",
            0xd5 => "PUSH D",
            0xd6 => "SUI",
            0xd7 => "RST 2",
            0xd8 => "RC",
            0xd9 => "0xd9",
            0xda => "JC",
            0xdb => "IN",
            0xdc => "CC",
            0xdd => "0xdd",
            0xde => "SBI",
            0xdf => "RST 3",
            0xe0 => "RPO",
            0xe1 => "POP H",
            0xe2 => "JPO",
            0xe3 => "XTHL",
            0xe4 => "CPO",
            0xe5 => "PUSH H",
            0xe6 => "ANI",
            0xe7 => "RST 4",
            0xe8 => "RPE",
            0xe9 => "PCHL",
            0xea => "JPE",
            0xeb => "XCHG",
            0xec => "CPE",
            0xed => "0xed",
            0xee => "XRI",
            0xef => "RST 5",
            0xf0 => "RP",
            0xf1 => "POP PSW",
            0xf2 => "JP",
            0xf3 => "DI",
            0xf4 => "CP",
            0xf5 => "PUSH PSW",
            0xf6 => "ORI",
            0xf7 => "RST 6",
            0xf8 => "RM",
            0xf9 => "SPHL",
            0xfa => "JM",
            0xfb => "EI",
            0xfc => "CM",
            0xfd => "0xfd",
            0xfe => "CPI",
            0xff => "RST 7",
        };

        write!(f, "{}", op)
    }
}
