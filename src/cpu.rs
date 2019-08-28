use std::{fmt, mem};

use condition_codes::ConditionCodes;
use opcode::Opcode;
use memory::Memory;
use register::Register;
use pointer::Pointer;

#[derive(Default)]
pub struct Cpu {
    pub a: Register,
    pub b: Register,
    pub c: Register,
    pub d: Register,
    pub e: Register,
    pub h: Register,
    pub l: Register,
    pub sp: Pointer,
    pub pc: Pointer,
    pub memory: Memory,
    pub conditions: ConditionCodes,
    pub int_enable: bool,
}

pub trait Machine {
    fn input(&mut self, port: u8) -> u8;
    fn output(&mut self, port: u8, byte: u8);
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4}",
                    "a",   "bc", "de", "hl", "pc", "sp", "flags")?;

        write!(f,
                 "{:04x} {:02x}{:02x} {:02x}{:02x} {:02x}{:02x} {:04x} {:04x} {:?}",
                 *self.a,
                 *self.b, *self.c,
                 *self.d, *self.e,
                 *self.h, *self.l,
                 *self.pc,
                 *self.sp,
                 self.conditions,
       )
    }
}

// HELP GROUP
impl Cpu {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_into_rom(&mut self, memory: &[u8], position: u16) {
        self.memory.load(memory, position);
    }

    pub fn emulate<M: Machine>(&mut self, machine: &mut M) -> u8 {
        let pc = self.pc;
        let opcode = Opcode::from(self.memory[pc]);
        let mut jumped = false;


        match *opcode {
            0x00 | 0x08 | 0x20 | 0x28 | 0x30 | 0x38 => {}
            0x01 | 0x11 | 0x21 | 0x31 => self.lxi(*opcode),

            0x02 | 0x12 => self.stax(*opcode),

            0x03 | 0x13 | 0x23 | 0x33 => self.inx(*opcode),

            0x04 | 0x0c | 0x14 | 0x1c |
            0x24 | 0x2c | 0x34 | 0x3c => self.inr(*opcode),

            0x05 | 0x0d | 0x15 | 0x1d |
            0x25 | 0x2d | 0x35 | 0x3d => self.dcr(*opcode),

            0x06 | 0x0e | 0x16 | 0x1e |
            0x26 | 0x2e | 0x36 | 0x3e => self.mvi(*opcode),

            0x07 => self.rlc(),

            0x09 | 0x19 | 0x29 | 0x39 => self.dad(*opcode),

            0x0a | 0x1a => self.ldax(*opcode),

            0x0b | 0x1b | 0x2b | 0x3b => self.dcx(*opcode),

            0x0f => self.rrc(),

            0x17 => self.ral(),
            0x1f => self.rar(),

            0x22 => self.shld(),
            0x27 => self.daa(),
            0x2a => self.lhld(),
            0x2f => self.cma(),
            0x32 => self.sta(),
            0x3a => self.lda(),
            0x37 => self.stc(),
            0x3f => self.cmc(),

            0x40..=0x75 | 0x77..=0x7f => self.mov(*opcode),

            0x76 => ::std::process::exit(0),

            0x80..=0x87 => self.add(*opcode),
            0x88..=0x8f => self.adc(*opcode),
            0x90..=0x97 => self.sub(*opcode),
            0x98..=0x9f => self.sbb(*opcode),

            0xa0..=0xa7 => self.ana(*opcode),
            0xa8..=0xaf => self.xra(*opcode),

            0xb0..=0xb7 => self.ora(*opcode),

            0xb8..=0xbf => self.cmp(*opcode),

            0xc0 => jumped = self.rnz(),

            0xc1 | 0xd1 | 0xe1 | 0xf1 => self.pop(*opcode),

            0xc2 => jumped = self.jnz(),
            0xc3 => jumped = self.jmp(),
            0xc4 => jumped = self.cnz(),

            0xc5 | 0xd5 | 0xe5 | 0xf5 => self.push(*opcode),

            0xc6 => self.adi(),

            0xc7 | 0xcf | 0xd7 | 0xdf |
            0xe7 | 0xef | 0xf7 | 0xff => jumped = self.rst(*opcode),

            0xc8 => jumped = self.rz(),

            0xc9 | 0xd9 => jumped = self.ret(),
            0xca => jumped = self.jz(),
            0xcc => jumped = self.cz(),
            0xcd | 0xdd | 0xed | 0xfd => jumped = self.call(),
            0xce => self.aci(),

            0xd0 => jumped = self.rnc(),
            0xd2 => jumped = self.jnc(),
            0xd3 => machine.output(self.get_d8(), *self.a),
            0xd4 => jumped = self.cnc(),
            0xd6 => self.sui(),
            0xd8 => jumped = self.rc(),
            0xda => jumped = self.jc(),
            0xdb => self.a = machine.input(self.get_d8()).into(),
            0xdc => jumped = self.cc(),
            0xde => self.sbi(),

            0xe0 => jumped = self.rpo(),
            0xe2 => jumped = self.jpo(),
            0xe3 => self.xthl(),
            0xe4 => jumped = self.cpo(),
            0xe6 => self.ani(),
            0xe8 => jumped = self.rpe(),
            0xe9 => jumped = self.pchl(),
            0xea => jumped = self.jpe(),
            0xeb => self.xchg(),
            0xec => jumped = self.cpe(),
            0xee => self.xri(),

            0xf0 => jumped = self.rp(),
            0xf2 => jumped = self.jp(),
            0xf3 => self.di(),
            0xf4 => jumped = self.cp(),
            0xf6 => self.ori(),
            0xf8 => jumped = self.rm(),
            0xf9 => self.sphl(),
            0xfa => jumped = self.jm(),
            0xfb => self.ei(),
            0xfc => jumped = self.cm(),
            0xfe => self.cpi(),

            code => {
                panic!("Unimplemented INSTRUCTION {:?}", Opcode::from(code));
            }

        }

        if !jumped {
            self.pc += opcode.size() as u16;
        }

        opcode.cycle_size()
    }

    fn get_offset(&self) -> u8 {
        let offset = ((self.h.to_u16()) << 8) | self.l.to_u16();
        self.memory[offset]
    }

    fn set_offset<I: Into<u8>>(&mut self, value: I) {
        let offset = ((self.h.to_u16()) << 8) | self.l.to_u16();
        self.memory.write(offset, value);
    }

    fn get_d8(&self) -> u8 {
        self.memory[self.pc + 1]
    }

    fn get_d16(&self) -> u16 {
        (self.memory[self.pc + 2] as u16) << 8 | self.memory[self.pc + 1] as u16
    }

    pub fn print_opcode(&self) {
        let pc = self.pc;
        let opcode = Opcode::from(self.memory[pc]);

        if opcode.size() == 1 {
            println!("{:04x} {:?}", *pc, opcode);
        } else if opcode.size() == 2 {
            println!("{:04x} {:?} {:02x}", *pc, opcode, self.memory[pc+1]);
        } else {
            println!("{:04x} {:?} {:02x}{:02x}",
                     *pc,
                     opcode,
                     self.memory[pc+2],
                     self.memory[pc+1]);
        }

    }

}

// DATA TRANSFER GROUP
impl Cpu {

    fn mov(&mut self, code: u8) {
        macro_rules! mov {
            ($set:ident ,
             $b:expr ,
             $c:expr ,
             $d:expr ,
             $e:expr ,
             $h:expr ,
             $l:expr ,
             $hl:expr ,
             $a:expr) => {
                self.$set = match code {
                    $b => self.b,
                    $c => self.c,
                    $d => self.d,
                    $e => self.e,
                    $h => self.h,
                    $l => self.l,
                    $hl => self.get_offset().into(),
                    $a => self.a,
                    _ => unreachable!(),
                };

            }
        }

        macro_rules! movm {
            ($b:expr ,
             $c:expr ,
             $d:expr ,
             $e:expr ,
             $h:expr ,
             $l:expr ,
             $a:expr) => {{
                let content = match code {
                    $b => self.b,
                    $c => self.c,
                    $d => self.d,
                    $e => self.e,
                    $h => self.h,
                    $l => self.l,
                    $a => self.a,
                    _ => unreachable!(),
                };

                self.set_offset(content);
            }}
        }

        match code {
            // B
            0x40..=0x47 => mov!(b, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47),
            0x48..=0x4f => mov!(c, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f),
            0x50..=0x57 => mov!(d, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57),
            0x58..=0x5f => mov!(e, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f),
            0x60..=0x67 => mov!(h, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67),
            0x68..=0x6f => mov!(l, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f),
            0x70..=0x75 | 0x77 => movm!(0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x77),
            0x78..=0x7f => mov!(a, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f),
            _ => unreachable!(),
        }
    }

    fn mvi(&mut self, code: u8) {
        macro_rules! mvi {
            ($x:ident) => {{
                self.$x = self.get_d8().into();
            }}
        }

        match code {
            0x06 => mvi!(b),
            0x0e => mvi!(c),
            0x16 => mvi!(d),
            0x1e => mvi!(e),
            0x26 => mvi!(h),
            0x2e => mvi!(l),
            0x36 => {
                let byte = self.get_d8();
                self.set_offset(byte);
            },
            0x3e => mvi!(a),
            _ => unreachable!(),
        }
    }

    fn lxi(&mut self, code: u8) {
        macro_rules! lxi {
            ($x:ident $y:ident) => {{
                self.$x = (self.get_d16() >> 8).into();
                self.$y = self.get_d8().into();
            }}
        }

        match code {
            0x01 => lxi!(b c),
            0x11 => lxi!(d e),
            0x21 => lxi!(h l),
            0x31 => {
                self.sp = self.get_d16().into();
            }
            _ => unreachable!(),
        }
    }

    fn ldax(&mut self, code: u8) {
        let offset = if code == 0x0a {
            (self.b.to_u16()) << 8 | self.c.to_u16()
        } else {
            (self.d.to_u16()) << 8 | self.e.to_u16()
        };

        self.a = self.memory[offset].into();
    }

    fn stax(&mut self, code: u8) {
        macro_rules! stax {
            ($x:ident $y:ident) => {{
                let x = (self.$x.to_u16() << 8) | self.$y.to_u16();
                self.memory.write(x, self.a);
            }}
        }
        match code {
            0x02 => stax!(b c),
            0x12 => stax!(d e),
            _ => unreachable!(),
        }
    }

    fn sphl(&mut self) {
        self.sp = (((*self.h as u16) << 8) | *self.l as u16).into();
    }

    fn pchl(&mut self) -> bool {
        self.pc = (((*self.h as u16) << 8) | *self.l as u16).into();
        true
    }

    fn xri(&mut self) {
        let rhs = self.get_d8();
        let lhs = self.a;
        self.a = (*lhs ^ rhs).into();
        self.conditions.set_all(self.a.to_u16(), *self.a);
    }

    fn xchg(&mut self) {
        mem::swap(&mut self.h, &mut self.d);
        mem::swap(&mut self.l, &mut self.e);
    }

    fn xthl(&mut self) {
        let new_h = self.memory[self.sp + 1];
        let new_l = self.memory[self.sp];

        let old_h = *self.h;
        let old_l = *self.l;

        self.h = new_h.into();
        self.l = new_l.into();

        self.memory.write(self.sp + 1, old_h);
        self.memory.write(self.sp, old_l);
    }
}

macro_rules! get_adrs {
    ($this:ident,
     $code:expr,
     $b:expr,
     $c:expr,
     $d:expr,
     $e:expr,
     $h:expr,
     $l:expr,
     $m:expr,
     $a:expr) => {
        match $code {
            $b => $this.b,
            $c => $this.c,
            $d => $this.d,
            $e => $this.e,
            $h => $this.h,
            $l => $this.l,
            $m => $this.get_offset().into(),
            $a => $this.a,
            _ => unreachable!(),
        }
    }
}

macro_rules! update_address {
    ($this:ident.$addr:ident $operand:tt $value:expr) => {{
        $this.$addr $operand $value;
        $this.conditions.set_all_except_carry($this.$addr.to_u16(), *$this.$addr);
    }}
}

macro_rules! update_pair {
    ($this:ident $x:ident $op:tt $y:ident) => {{
        let mut answer = (($this.$x.to_u16()) << 8) | $this.$y.to_u16();
        answer = answer.$op(1);

        $this.$x = (answer >> 8).into();
        $this.$y = answer.into();
    }}
}

// ARITHMETIC GROUP
impl Cpu {

    fn add(&mut self, code: u8) {
        let rhs = get_adrs!(self, code, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87);
        let lhs = self.a;
        let answer = lhs.to_u16() + rhs.to_u16();

        self.a = answer.into();

        self.conditions.set_all(answer, (*lhs & 0xf) + (*rhs & 0xf));
    }

    fn adc(&mut self, code: u8) {
        let lhs = self.a.to_u16();
        let addr = get_adrs!(self, code, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f);
        let rhs = addr.to_u16() + self.conditions.cy as u16;

        let answer = lhs.wrapping_add(rhs);

        self.a = answer.into();
        self.conditions.set_all(answer, (lhs as u8 & 0xf).wrapping_add(rhs as u8 & 0xf));
    }

    fn adi(&mut self) {
        let lhs = self.a.to_u16();
        let rhs = self.get_d8() as u16;
        let answer = lhs.wrapping_add(rhs);

        self.conditions.set_all(answer, (lhs as u8 & 0xf).wrapping_add(rhs as u8 & 0xf));
        self.a = answer.into();
    }

    fn aci(&mut self) {
        let lhs = self.a.to_u16();
        let rhs = self.get_d8().wrapping_add(self.conditions.cy as u8) as u16;
        let answer = lhs.wrapping_add(rhs);

        self.conditions.set_all(answer, (lhs as u8 & 0xf).wrapping_add(rhs as u8 & 0xf));
        self.a = answer.into();

    }

    fn cmp(&mut self, code: u8) {
        let lhs = self.a.to_u16();
        let rhs = get_adrs!(self, code, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf);

        let answer = lhs.wrapping_sub(rhs.to_u16());

        self.conditions.set_all(answer, (lhs & 0xf).wrapping_sub(rhs.to_u16() & 0xf) as u8);
    }

    fn sub(&mut self, code: u8) {
        let lhs = self.a.to_u16();
        let rhs = get_adrs!(self, code, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97);
        let answer = lhs.wrapping_sub(rhs.to_u16());

        self.a = answer.into();
        self.conditions.set_all(answer, (lhs & 0xf).wrapping_sub(rhs.to_u16() & 0xf) as u8);
    }

    fn sbb(&mut self, code: u8) {
        let lhs = self.a.to_u16();
        let addr = get_adrs!(self, code, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f);
        let rhs = addr.to_u16().wrapping_add(self.conditions.cy as u16);

        let answer = lhs.wrapping_sub(rhs);
        self.a = answer.into();
        self.conditions.set_all(answer, (lhs & 0xf).wrapping_sub(rhs & 0xf) as u8);
    }

    fn sui(&mut self) {
        let lhs = self.a.to_u16();
        let rhs = self.get_d8() as u16;
        let answer = lhs.wrapping_sub(rhs);
        self.conditions.set_all(answer, (lhs & 0xf).wrapping_sub(rhs & 0xf) as u8);
        self.a = answer.into();
    }

    fn sbi(&mut self) {
        let lhs = self.a.to_u16();

        let rhs = self.get_d8().wrapping_add(self.conditions.cy as u8) as u16;

        let answer = self.a.to_u16().wrapping_sub(rhs);

        self.conditions.set_all(answer, (lhs & 0xf).wrapping_sub(rhs & 0xf) as u8);
        self.a = answer.into();
    }

    fn inr(&mut self, code: u8) {

        match code {
            0x04 => update_address!(self.b += 1),
            0x0c => update_address!(self.c += 1),
            0x14 => update_address!(self.d += 1),
            0x1c => update_address!(self.e += 1),
            0x24 => update_address!(self.h += 1),
            0x2c => update_address!(self.l += 1),
            0x34 => {
                let lhs = self.get_offset();
                let value = lhs.wrapping_add(1);
                self.conditions.set_all_except_carry(value as u16, (lhs & 0xf) + 1);
                self.set_offset(value);
            },
            0x3c => update_address!(self.a += 1),
            _ => unreachable!(),
        };
    }

    fn dcr(&mut self, code: u8) {

        match code {
            0x05 => update_address!(self.b -= 1),
            0x0d => update_address!(self.c -= 1),
            0x15 => update_address!(self.d -= 1),
            0x1d => update_address!(self.e -= 1),
            0x25 => update_address!(self.h -= 1),
            0x2d => update_address!(self.l -= 1),
            0x35 => {
                let value = self.get_offset().wrapping_sub(1);
                self.conditions.set_all_except_carry(value as u16, value);
                self.set_offset(value);
            },
            0x3d => update_address!(self.a -= 1),
            _ => unreachable!(),
        };
    }

    fn inx(&mut self, code: u8) {
        match code {
            0x03 => update_pair!(self b wrapping_add c),
            0x13 => update_pair!(self d wrapping_add e),
            0x23 => update_pair!(self h wrapping_add l),
            0x33 => self.sp += 1,
            _ => unreachable!(),
        }
    }

    fn dcx(&mut self, code: u8) {
        match code {
            0x0b => update_pair!(self b wrapping_sub c),
            0x1b => update_pair!(self d wrapping_sub e),
            0x2b => update_pair!(self h wrapping_sub l),
            0x3b => self.sp -= 1,
            _ => unreachable!(),
        }
    }

    fn dad(&mut self, code: u8) {
        macro_rules! dad {
            ($x:ident, $y:ident) => {{
                let hl = ((self.h.to_u16()) << 8) | self.l.to_u16();
                let second = (self.$x.to_u16()) << 8 | self.$y.to_u16();

                let answer = hl.wrapping_add(second);

                self.conditions.set_cy(answer);
                self.h = (answer >> 8).into();
                self.l = answer.into();
            }}
        }

        match code {
            0x09 => dad!(b, c),
            0x19 => dad!(d, e),
            0x29 => dad!(h, l),
            0x39 => {
                let mut hl = (self.h.to_u16() << 8) | self.l.to_u16();
                hl = hl.wrapping_add(*self.sp);

                self.conditions.set_cy(hl);
                self.h = (hl >> 8).into();
                self.l = hl.into();
            },
            _ => unreachable!(),
        }
    }

    fn daa(&mut self) {
        let mut answer = self.a.to_u16();

        let least = answer & 0xf;

        if self.conditions.ac || least > 9 {
            answer += 6;

            if answer & 0xf < least {
                self.conditions.ac = true;
            }
        }

        let least = answer & 0xf;
        let mut most = (answer >> 4) & 0xf;

        if self.conditions.cy || most > 9 {
            most += 6;
        }

        let answer = ((most << 4) as u16) | least as u16;
        self.conditions.set_all_except_ac(answer);

        self.a = answer.into();
    }
}


// BRANCH GROUP
impl Cpu {
    fn jmp(&mut self) -> bool {
        if cfg!(feature = "cpudiag") && self.get_d16() == 0 {
            println!();
            ::std::process::exit(0);
        }
        self.jump();
        true
    }

    fn jnc(&mut self) -> bool {
        if !self.conditions.cy {
            self.jump();
        }

        !self.conditions.cy
    }

    fn jc(&mut self) -> bool {
        if self.conditions.cy {
            self.jump();
        }
        self.conditions.cy
    }

    fn jp(&mut self) -> bool {
        if !self.conditions.s {
            self.jump();
        }

        !self.conditions.s
    }

    fn jpo(&mut self) -> bool {
        if !self.conditions.p {
            self.jump();
        }

        !self.conditions.p
    }

    fn jpe(&mut self) -> bool {
        if self.conditions.p {
            self.jump()
        }

        self.conditions.p
    }

    fn jz(&mut self) -> bool {
        if self.conditions.z {
            self.jump()
        }
        self.conditions.z
    }

    fn jnz(&mut self) -> bool {
        if !self.conditions.z {
            self.jump()
        }
        !self.conditions.z
    }

    fn jm(&mut self) -> bool {
        if self.conditions.s {
            self.jump()
        }
        self.conditions.s
    }

    fn call(&mut self) -> bool {
        use std::char;

        if cfg!(feature = "cpudiag") && self.get_d16() == 5 && self.c == 2 {
            let letter = (self.d.to_u16() << 8) | self.e.to_u16();
            print!("{}", char::from_u32(letter as u32).unwrap());
            false
        } else if cfg!(feature = "cpudiag") && self.get_d16() == 0 {
            ::std::process::exit(0)
        } else {
            let ret = *self.pc + 3;
            self.memory.write(self.sp - 1, (ret >> 8) as u8);
            self.memory.write(self.sp - 2, ret as u8);
            self.sp -= 2;
            self.jump();
            true
        }
    }

    fn cz(&mut self) -> bool {
        if self.conditions.z {
            self.call();
        }

        self.conditions.z
    }

    fn cnz(&mut self) -> bool {
        if !self.conditions.z {
            self.call();
        }

        !self.conditions.z
    }

    fn cc(&mut self) -> bool {
        if self.conditions.cy {
            self.call();
        }

        self.conditions.cy
    }

    fn cnc(&mut self) -> bool {
        if !self.conditions.cy {
            self.call();
        }

        !self.conditions.cy
    }

    fn cpe(&mut self) -> bool {
        if self.conditions.p {
            self.call();
        }

        self.conditions.p
    }

    fn cpo(&mut self) -> bool {
        if !self.conditions.p {
            self.call();
        }

        !self.conditions.p
    }

    fn cm(&mut self) -> bool {
        if self.conditions.s {
            self.call();
        }

        self.conditions.s
    }

    fn cp(&mut self) -> bool {
        if !self.conditions.s {
            self.call();
        }

        !self.conditions.s
    }

    fn jump(&mut self) {
        self.pc = self.get_d16().into();
    }

    fn rc(&mut self) -> bool {
        if self.conditions.cy {
            self.ret();
        }

        self.conditions.cy
    }

    fn rnc(&mut self) -> bool {
        if !self.conditions.cy {
            self.ret();
        }

        !self.conditions.cy
    }

    fn rz(&mut self) -> bool {
        if self.conditions.z {
            self.ret();
        }
        self.conditions.z
    }

    fn rnz(&mut self) -> bool {
        if !self.conditions.z {
            self.ret();
        }
        !self.conditions.z
    }

    fn rm(&mut self) -> bool {
        if self.conditions.s {
            self.ret();
        }

        self.conditions.s
    }

    fn rp(&mut self) -> bool {
        if !self.conditions.s {
            self.ret();
        }

        !self.conditions.s
    }

    fn rpe(&mut self) -> bool {
        if self.conditions.p {
            self.ret();
        }

        self.conditions.p
    }

    fn rpo(&mut self) -> bool {
        if !self.conditions.p {
            self.ret();
        }

        !self.conditions.p
    }

    fn ret(&mut self) -> bool {
        let low = self.memory[self.sp] as u16;
        let high = self.memory[self.sp + 1] as u16;

        self.pc = ((high << 8) | low).into();
        self.sp += 2;
        true
    }
}

// LOGICAL GROUP
impl Cpu {
    fn ani(&mut self) {
        let answer = (*self.a & self.get_d8()) as u16;

        self.conditions.set_cy(answer);
        self.conditions.set_z(answer);
        self.conditions.set_s(answer);
        self.conditions.set_p(answer);

        self.a = answer.into();
    }

    fn ana(&mut self, code: u8) {
        let lhs = self.a;
        let rhs = match code {
            0xa0 => self.b,
            0xa1 => self.c,
            0xa2 => self.d,
            0xa3 => self.e,
            0xa4 => self.h,
            0xa5 => self.l,
            0xa6 => self.get_offset().into(),
            0xa7 => self.a,
            _ => unreachable!(),
        };

        let answer = lhs & rhs;

        self.a = answer;

        self.conditions.set_all(answer.to_u16(), *answer);
    }

    fn rlc(&mut self) {
        self.a = self.a.0.rotate_left(1).into();

        self.conditions.cy = (self.a & 1) != 0;
    }

    fn ral(&mut self) {
        let new_carry = self.a & 0x80 != 0;

        self.a = (self.a << 1) | self.conditions.cy as u8;

        self.conditions.cy = new_carry;
    }

    fn rar(&mut self) {
        let new_carry = self.a & 1 != 0;

        self.a = (self.a >> 1) | ((self.conditions.cy as u8) << 7);

        self.conditions.cy = new_carry;
    }

    fn rrc(&mut self) {
        self.a = self.a.0.rotate_right(1).into();

        self.conditions.cy = (self.a & 0x80) != 0;
    }

    fn ora(&mut self, code: u8) {
        self.a |= match code {
            0xb0 => self.b,
            0xb1 => self.c,
            0xb2 => self.d,
            0xb3 => self.e,
            0xb4 => self.h,
            0xb5 => self.l,
            0xb6 => self.get_offset().into(),
            0xb7 => self.a,
            _ => unreachable!(),
        };

        self.conditions.set_all(self.a.to_u16(), 0);
    }

    fn ori(&mut self) {
        let answer = self.a.to_u16() | self.get_d8() as u16;

        self.conditions.set_cy(answer);
        self.conditions.set_z(answer);
        self.conditions.set_s(answer);
        self.conditions.set_p(answer);

        self.a = answer.into();
    }

    fn stc(&mut self) {
        self.conditions.cy = true;
    }

    fn cma(&mut self) {
        self.a = (!*self.a).into();
    }

    fn cmc(&mut self) {
        self.conditions.cy = !self.conditions.cy;
    }

    fn shld(&mut self) {
        let addr = self.get_d16();

        self.memory.write(addr, self.l);
        self.memory.write(addr + 1, *self.h);
    }

    fn lhld(&mut self) {
        let addr = self.get_d16();

        self.l = self.memory[addr].into();
        self.h = self.memory[addr + 1].into();
    }

    fn lda(&mut self) {
        self.a = self.memory[self.get_d16()].into();
    }

    fn sta(&mut self) {
        let adr = self.get_d16();
        self.memory.write(adr, self.a);
    }

    fn cpi(&mut self) {
        let byte = self.get_d8() as u16;
        let answer = self.a.to_u16().wrapping_sub(byte);

        self.conditions.set_z(answer);
        self.conditions.cy = self.a.to_u16() < byte;
        self.conditions.set_p(answer);
        self.conditions.set_s(answer);
        self.conditions.set_ac((self.a & 0xf).wrapping_sub(byte as u8 & 0xf));
    }

    fn xra(&mut self, code: u8) {
        let rhs = match code {
            0xa8 => self.b,
            0xa9 => self.c,
            0xaa => self.d,
            0xab => self.e,
            0xac => self.h,
            0xad => self.l,
            0xae => self.get_offset().into(),
            0xaf => self.a,
            _ => unreachable!(),
        };

        let lhs = self.a;
        self.a = (*lhs ^ *rhs).into();

        self.conditions.set_all(self.a.to_u16(), (*lhs & 0xf) ^ (*rhs & 0xf));
    }
}

// IO GROUP
impl Cpu {

    fn rst(&mut self, code: u8) -> bool {
        let ret = self.pc;
        self.memory.write(self.sp - 1, (ret >> 8));
        self.memory.write(self.sp - 2, ret);
        self.sp -= 2;
        self.pc = (code & 0x38).into();
        true
    }

    pub fn interrupt(&mut self, code: u8) {
        self.rst(code);
        self.int_enable = false;
    }

    fn push(&mut self, code: u8) {
        macro_rules! push {
            ($x:ident $y:ident) => {{
                self.memory.write(self.sp - 1, self.$x);
                self.memory.write(self.sp - 2, self.$y);
                self.sp -= 2;
            }}
        }
        match code {
            0xc5 => push!(b c),
            0xd5 => push!(d e),
            0xe5 => push!(h l),
            0xf5 => push!(a conditions),
            _ => unreachable!(),
        }
    }

    fn pop(&mut self, code: u8) {
        macro_rules! pop {
            ($x:ident $y:ident) => {{
                self.$y = self.memory[self.sp].into();
                self.$x = self.memory[self.sp + 1].into();
                self.sp += 2;
            }}
        }
        match code {
            0xc1 => pop!(b c),
            0xd1 => pop!(d e),
            0xe1 => pop!(h l),
            0xf1 => pop!(a conditions),
            _ => unreachable!(),
        }
    }

    fn ei(&mut self) {
        self.int_enable = true;
    }

    fn di(&mut self) {
        self.int_enable = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Facade;

    impl Machine for Facade {
        fn input(&mut self, _: u8) -> u8 {0}

        fn output(&mut self, _: u8, _: u8) {}
    }

    #[test]
    fn cpi() {
        let mut cpu = Cpu::new();

        cpu.load_into_rom(&[0xfe, 0x40], 0);

        cpu.a = 0x4au8.into();
        cpu.conditions.z = true;
        cpu.conditions.cy = true;

        cpu.emulate(&mut Facade);

        assert!(!cpu.conditions.z);
        assert!(!cpu.conditions.cy);
    }

    #[test]
    fn rrc() {
        let mut cpu = Cpu::new();

        cpu.a = 0xf2u8.into();
        cpu.load_into_rom(&[0x0f], 0);
        cpu.emulate(&mut Facade);

        assert_eq!(cpu.a, 0x79);
    }

    #[test]
    fn adi() {
        let mut cpu = Cpu::new();

        cpu.a = 0u16.into();
        cpu.load_into_rom(&[0xc6, 6], 0);
        cpu.emulate(&mut Facade);

        assert_eq!(cpu.a, 6);
        assert!(!cpu.conditions.cy);
        assert!(cpu.conditions.p);
        assert!(!cpu.conditions.s);
        assert!(!cpu.conditions.z);
    }

    #[test]
    fn daa() {
        let mut cpu = Cpu::new();

        cpu.a = 0x9bu16.into();
        cpu.load_into_rom(&[0x27], 0);

        cpu.emulate(&mut Facade);

        assert_eq!(cpu.a, 1);

        assert!(cpu.conditions.ac);
        assert!(cpu.conditions.cy);

    }

    #[test]
    fn ral() {
        let mut cpu = Cpu::new();

        cpu.a = 0xb5u16.into();
        cpu.load_into_rom(&[0x17], 0);

        cpu.emulate(&mut Facade);

        assert_eq!(cpu.a, 0x6a);
        assert!(cpu.conditions.cy);

    }

    #[test]
    fn rlc() {
        let mut cpu = Cpu::new();

        cpu.load_into_rom(&[0x07], 0);

        cpu.a = 0xf2u8.into();

        cpu.emulate(&mut Facade);

        assert_eq!(*cpu.a, 0xe5);
        assert!(cpu.conditions.cy);
    }

    #[test]
    fn rar() {
        let mut cpu = Cpu::new();

        cpu.load_into_rom(&[0x1f], 0);

        cpu.a = 0x6au8.into();
        cpu.conditions.cy = true;

        cpu.emulate(&mut Facade);

        assert_eq!(*cpu.a, 0xb5);
        assert!(!cpu.conditions.cy);
    }

    #[test]
    fn push_pop() {
        macro_rules! push_pop {
            ($x:ident $y:ident, $push:expr, $pop:expr) => {{
                let mut cpu = Cpu::new();

                let $x = 7u8.into();
                let $y = 9u8.into();

                cpu.$x = $x;
                cpu.$y = $y;

                cpu.load_into_rom(&[$push, $pop], 0);
                cpu.emulate(&mut Facade);

                cpu.$x = 0u8.into();
                cpu.$y = 0u8.into();

                cpu.emulate(&mut Facade);

                assert_eq!(cpu.$x, $x);
                assert_eq!(cpu.$y, $y);
            }}
        }

        push_pop!(b c, 0xc5, 0xc1);
        push_pop!(d e, 0xd5, 0xd1);
        push_pop!(h l, 0xe5, 0xe1);
        push_pop!(a conditions, 0xf5, 0xf1);
    }

}
