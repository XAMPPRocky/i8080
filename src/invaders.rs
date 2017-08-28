use std::time::Instant;
use std::{cmp, mem};

use ears::{AudioController, Sound};
use piston_window::{Input, Button, Key, ButtonArgs, ButtonState};

use cpu::{Cpu, Machine};

const FILE_POSITIONS: [(&'static [u8; 2048], u16); 4] = [
    (include_bytes!("../games/invaders/src/invaders.h"), 0),
    (include_bytes!("../games/invaders/src/invaders.g"), 0x800),
    (include_bytes!("../games/invaders/src/invaders.f"), 0x1000),
    (include_bytes!("../games/invaders/src/invaders.e"), 0x1800),
];

thread_local! {
}

const CREDIT: u8 = 0x1;
const FIRE: u8 = 0x10;
const LEFT: u8 = 0x20;
const P1_START: u8 = 0x4;
const P2_START: u8 = 0x2;
const RIGHT: u8 = 0x40;

struct Sounds {
    ufo: Sound,
    shot: Sound,
    flash: Sound,
    enemy_death: Sound,
    first_movement: Sound,
    second_movement: Sound,
    third_movement: Sound,
    fourth_movement: Sound,
    ufo_hit: Sound,
}

impl Sounds {
    fn new() -> Self {
        Sounds {
            ufo: {
                let mut sound = Sound::new("games/invaders/sound/0.wav").expect("0.wav not found");
                sound.set_looping(true);
                sound
            },
            shot: Sound::new("games/invaders/sound/1.wav").expect("1.wav not found"),
            flash: Sound::new("games/invaders/sound/2.wav").expect("2.wav not found"),
            enemy_death: Sound::new("games/invaders/sound/3.wav").expect("3.wav not found"),
            first_movement: Sound::new("games/invaders/sound/4.wav").expect("4.wav not found"),
            second_movement: Sound::new("games/invaders/sound/5.wav").expect("5.wav not found"),
            third_movement: Sound::new("games/invaders/sound/6.wav").expect("6.wav not found"),
            fourth_movement: Sound::new("games/invaders/sound/7.wav").expect("7.wav not found"),
            ufo_hit: Sound::new("games/invaders/sound/8.wav").expect("8.wav not found"),
        }
    }
}

pub struct SpaceInvaders {
    cpu: Cpu,
    first_port: u8,
    interrupt_num: bool,
    last_port_five: u8,
    last_port_three: u8,
    next_interrupt: i64,
    overnanos: u64,
    previous: Instant,
    second_port: u8,
    shift_offset: u16,
    shiftx: u8,
    shifty: u8,
    sounds: Sounds,
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
            next_interrupt: 0,
            interrupt_num: false,
            overnanos: 0,
            last_port_three: 0,
            last_port_five: 0,
            sounds: Sounds::new(),
        }
    }

    pub fn framebuffer(&self) -> &[u8] {
        &self.cpu.memory[0x2400..0x4000]
    }

    pub fn emulate(&mut self) {
        const NANOS_PER_SEC: u64 = 1_000_000_000;
        const HERTZ: u64 = 2_000_000;
        const NANOS_PER_CYCLE: u64 = NANOS_PER_SEC / HERTZ;
        const INTERRUPT_CYCLES: i64 = 2_000 * 8; //2_000_000Hz * 8ms

        let now = Instant::now();
        let duration = now.duration_since(self.previous);
        let nanos_needed = (duration.as_secs() * NANOS_PER_SEC) +
                           (duration.subsec_nanos() as u64);

        if nanos_needed <= self.overnanos {
            return;
        }

        // never execute more than 1 second worth of work at once
        let nanos_needed = cmp::min(nanos_needed - self.overnanos, NANOS_PER_SEC);
        let cycles_needed = (nanos_needed + NANOS_PER_CYCLE - 1) / NANOS_PER_CYCLE;
        let mut cycles_passed = 0u64;
        let mut cpu = mem::replace(&mut self.cpu, Cpu::new());

        while self.next_interrupt < cycles_needed as i64 {

            while self.next_interrupt > cycles_passed as i64  {
                cycles_passed += cpu.emulate(self) as u64;
            }

            self.try_interrupt(&mut cpu);
            self.next_interrupt += INTERRUPT_CYCLES;
        }

        while cycles_needed > cycles_passed  {
            cycles_passed += cpu.emulate(self) as u64;
        }

        self.next_interrupt -= cycles_passed as i64;
        self.overnanos = cycles_passed * NANOS_PER_CYCLE - nanos_needed;
        self.previous = now;
        self.cpu = cpu;
    }

    fn try_interrupt(&mut self, cpu: &mut Cpu) {
        self.interrupt_num = !self.interrupt_num;
        if !cpu.int_enable {
            return;
        }
        if self.interrupt_num {
            cpu.interrupt(8);
        } else {
            cpu.interrupt(16);
        }
    }

    pub fn handle_event(&mut self, event: &Input) {
        match *event {
            Input::Button(ButtonArgs {
                state: ButtonState::Press,
                button: Button::Keyboard(key),
                scancode: _,
            }) => self.keydown(key),
            Input::Button(ButtonArgs {
                state: ButtonState::Release,
                button: Button::Keyboard(key),
                scancode: _,
            }) => self.keyup(key),
            _ => {}
        }
    }

    fn keydown(&mut self, key: Key) {
        match key {
            Key::Left | Key::A => self.first_port |= LEFT,
            Key::C => self.first_port |= CREDIT,
            Key::Right | Key::D => self.first_port |= RIGHT,
            Key::Space | Key::F => self.first_port |= FIRE,
            Key::D1 => self.first_port |= P1_START,
            Key::D2 => self.first_port |= P2_START,
            _ => {}
        }
    }

    fn keyup(&mut self, key: Key) {
        match key {
            Key::Left | Key::A => self.first_port &= !LEFT,
            Key::C => self.first_port &= !CREDIT,
            Key::Right | Key::D => self.first_port &= !RIGHT,
            Key::Space | Key::F => self.first_port &= !FIRE,
            Key::D1 => self.first_port &= !P1_START,
            Key::D2 => self.first_port &= !P2_START,
            _ => {}
        }
    }
}

impl Machine for SpaceInvaders {
    fn input(&mut self, port: u8) -> u8 {
        match port {
            0 => 1,
            1 => self.first_port,
            2 => self.second_port,
            3 => {
                let value = ((self.shifty as u16) << 8) | self.shiftx as u16;
                (value >> (8 - self.shift_offset)) as u8
            }
            code => panic!("Unimplemented INPUT PORT {:?}", code),
        }
    }

    fn output(&mut self, port: u8, byte: u8) {
        macro_rules! play {
            ($sound:ident) => {
                if !self.sounds.$sound.is_playing() {
                    self.sounds.$sound.play();
                }
            }
        }

        match port {
            2 => {
                self.shift_offset = byte as u16;
            }
            3 => {
                macro_rules! changed {
                    ($position:expr) => {
                        (self.last_port_three & $position) ^ (byte & $position) > 0
                    }
                }

                if changed!(0x1) {
                    if !self.sounds.ufo.is_playing() {
                        self.sounds.ufo.play();
                    } else {
                        self.sounds.ufo.stop();
                    }
                }

                if changed!(0x2) {
                    play!(shot);
                }

                if changed!(0x4) {
                    play!(flash);
                }

                if changed!(0x8) {
                    play!(enemy_death);
                }

                self.last_port_three = byte;
            }
            4 => {
                self.shiftx = self.shifty;
                self.shifty = byte;
            }
            5 => {
                macro_rules! changed {
                    ($position:expr) => {
                        (self.last_port_five & $position) ^ (byte & $position) > 0
                    }
                }

                if changed!(0x1) {
                    play!(first_movement);
                }

                if changed!(0x2) {
                    play!(second_movement);
                }

                if changed!(0x4) {
                    play!(third_movement);
                }

                if changed!(0x8) {
                    play!(fourth_movement);
                }

                if changed!(0x10) {
                    play!(ufo_hit);
                }

                self.last_port_five = byte;
            }
            6 => {}

            code => panic!("Unimplemented OUTPUT PORT {:?}", code),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_hardware() {
        let mut machine = SpaceInvaders::new();

        machine.output(2, 2);

        machine.output(4, 0xff);
        machine.output(4, 0x3f);

        assert_eq!(machine.input(3), 0xff);
    }
}
