use std::mem;
use std::time::{Duration, Instant};

use piston_window::{Input, Button, Key};

use cpu::{Cpu, Machine};

const FILE_POSITIONS: [(&'static [u8; 2048], u16); 4] = [
    (include_bytes!("../inv/invaders.h"), 0),
    (include_bytes!("../inv/invaders.g"), 0x800),
    (include_bytes!("../inv/invaders.f"), 0x1000),
    (include_bytes!("../inv/invaders.e"), 0x1800),
];

pub struct SpaceInvaders {
    pub cpu: Cpu,
    shift_offset: u16,
    shiftx: u16,
    shifty: u16,
    first_port: u8,
    second_port: u8,
    previous: Instant,
    interrupt_num: u8,
    next_interrupt: Instant,
    overnanos: u64,
}

impl SpaceInvaders {
    pub fn new() -> Self {
        let mut cpu = Cpu::new();

        for &(file, position) in &FILE_POSITIONS {
            cpu.load_into_rom(file, position);
        }

        SpaceInvaders {
            cpu: cpu,
            shift_offset: 0,
            shiftx: 0,
            shifty: 0,
            first_port: 1,
            second_port: 0,
            previous: Instant::now(),
            next_interrupt: Instant::now(),
            interrupt_num: 1,
            overnanos: 0,
        }
    }

    pub fn framebuffer(&self) -> &[u8] {
        &self.cpu.memory[0x2400..0x4000]
    }

    pub fn emulate(&mut self) {
        const SLOWDOWN: u64 = 10;
        const NANOS_PER_SEC: u64 = 1_000_000_000;
        const HERTZ: u64 = 2_000_000 / SLOWDOWN;
        const NANOS_PER_CYCLE: u64 = NANOS_PER_SEC / HERTZ;
        const INTERRUPT_MILLIS: u64 = 8 * SLOWDOWN;

        let mut cpu = mem::replace(&mut self.cpu, Cpu::new());
        let now = Instant::now();

        if cpu.int_enable && now >= self.next_interrupt {
            if self.interrupt_num == 1 {
                cpu.interrupt(8);
                self.interrupt_num = 2;
            } else {
                cpu.interrupt(16);
                self.interrupt_num = 1;
            }
            self.next_interrupt = now + Duration::from_millis(INTERRUPT_MILLIS);
        }

        let duration = now.duration_since(self.previous);
        let nanos_needed = (duration.as_secs() * NANOS_PER_SEC) + (duration.subsec_nanos() as u64);

        if nanos_needed <= self.overnanos {
            self.cpu = cpu;
            return;
        }

        // never execute more than 1 second worth of work at once
        let nanos_needed = ::std::cmp::min(nanos_needed - self.overnanos, NANOS_PER_SEC);

        let cycles_needed = (nanos_needed + NANOS_PER_CYCLE - 1) / NANOS_PER_CYCLE;

        let mut cycles_passed = 0;
        while cycles_needed > cycles_passed  {
            cycles_passed += cpu.emulate(self) as u64;
        }

        self.overnanos = cycles_passed * NANOS_PER_CYCLE - nanos_needed;
        self.previous = now;
        self.cpu = cpu;
    }

    pub fn handle_event(&mut self, event: &Input) {
        match *event {
            Input::Press(Button::Keyboard(key)) => self.keydown(key),
            Input::Release(Button::Keyboard(key)) => self.keyup(key),
            _ => {}
        }
    }

    fn keydown(&mut self, _: Key) { }
    fn keyup(&mut self, _: Key) { }
}

impl Machine for SpaceInvaders {
    fn input(&mut self, port: u8) -> u8 {
        match port {
            0 => 1,
            1 => self.first_port,
            2 => self.second_port,
            3 => {
                let value = (self.shifty << 8) | self.shiftx;
                (value >> (8 - self.shift_offset)) as u8
            }
            code => panic!("Unimplemented INPUT PORT {:?}", code),
        }
    }

    fn output(&mut self, port: u8, byte: u8) {
        match port {
            2 => {
                self.shift_offset = (byte & 0x7) as u16;
            }
            3 => {}
            4 => {
                self.shiftx = self.shifty;
                self.shifty = byte as u16;
            }
            5 => {}
            6 => {}

            code => panic!("Unimplemented OUTPUT PORT {:?}", code),
        }
    }
}

