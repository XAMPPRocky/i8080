use kira::{manager::AudioManager, sound::handle::SoundHandle};
use winit::{dpi::LogicalSize, event::{KeyboardInput, VirtualKeyCode, Event, WindowEvent}, event_loop::{EventLoop, ControlFlow}, window::WindowBuilder};
use pixels::{Pixels, SurfaceTexture};
use i8080::*;

use std::time::Instant;
use std::{cmp, mem};

const WIDTH: u32 = 224;
const HEIGHT: u32 = 256;
const FILE_POSITIONS: [(&'static [u8; 2048], u16); 4] = [
    (include_bytes!("../../games/invaders/invaders.h"), 0),
    (include_bytes!("../../games/invaders/invaders.g"), 0x800),
    (include_bytes!("../../games/invaders/invaders.f"), 0x1000),
    (include_bytes!("../../games/invaders/invaders.e"), 0x1800),
];

const CREDIT: u8 = 0x1;
const FIRE: u8 = 0x10;
const LEFT: u8 = 0x20;
const P1_START: u8 = 0x4;
const P2_START: u8 = 0x2;
const RIGHT: u8 = 0x40;

struct Sounds {
    #[allow(unused)]
    manager: AudioManager,
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
        let mut manager = AudioManager::new(<_>::default()).unwrap();
        Self {
            ufo: Sound::new("games/invaders/sounds/ufo_lowpitch.wav", &mut manager),
            shot: Sound::new("games/invaders/sounds/shoot.wav", &mut manager),
            flash: Sound::new("games/invaders/sounds/explosion.wav", &mut manager),
            enemy_death: Sound::new("games/invaders/sounds/invaderkilled.wav", &mut manager),
            first_movement: Sound::new("games/invaders/sounds/fastinvader1.wav", &mut manager),
            second_movement: Sound::new("games/invaders/sounds/fastinvader2.wav", &mut manager),
            third_movement: Sound::new("games/invaders/sounds/fastinvader3.wav", &mut manager),
            fourth_movement: Sound::new("games/invaders/sounds/fastinvader4.wav", &mut manager),
            ufo_hit: Sound::new("games/invaders/sounds/ufo_lowpitch.wav", &mut manager),
            manager,
        }
    }
}

pub struct Sound {
    sound: SoundHandle,
    instance: Option<kira::instance::handle::InstanceHandle>
}

impl Sound {
    pub fn new<A: AsRef<std::path::Path>>(path: A, manager: &mut AudioManager) -> Self {
        let sound = manager.load_sound(path, <_>::default()).unwrap();

        Self {
            sound,
            instance: None,
        }
    }

    pub fn is_playing(&self) -> bool {
        !self.instance.as_ref().filter(|instance| instance.state() != kira::instance::InstanceState::Stopped).is_none()
    }

    pub fn play(&mut self) {
        self.instance.replace(self.sound.play(<_>::default()).unwrap());
    }

    pub fn stop(&mut self) {
        self.sound.stop(<_>::default()).unwrap()
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

    fn update(&mut self, frame: &mut [u8]) {
        self.emulate();
        let mut frame = frame.chunks_exact_mut(4).collect::<Vec<&mut [u8]>>();
        let framebuffer = self.framebuffer();

        let mut i = 0;
        for x in 0..WIDTH {
            for y in (0..HEIGHT).step_by(8) {
                let byte = framebuffer[i];
                i += 1;

                for shift in 0..8 {
                    let result = if (byte >> shift) & 1 == 0 {
                        [0, 0, 0, 255]
                    } else if y <= 63 && (y >= 15 || y <= 15 && x >= 20 && x <= 120) {
                        [0, 255, 0, 255]
                    } else if y >= 200 && y <= 220 {
                        [255, 0, 0, 255]
                    } else {
                        [255; 4]
                    };

                    frame[(((HEIGHT - 1 - y) * WIDTH) + x - (WIDTH * shift)) as usize].copy_from_slice(&result);
                }
            }
        }

        //     // Really x is y and y is x as the frame is rotated 90 degrees
        //     let y = i * 8 / (WIDTH as usize + 1);
        //     for shift in 0..SHIFT_END + 1 {
        //         let x = ((i * 8) % (WIDTH as usize)) + shift as usize;


        //         let result = &[x as u8, x as u8, x as u8, x as u8][..];

        //         frame[x - y].copy_from_slice(&result);
        //     }
        // }
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

    pub fn handle_event(&mut self, event: KeyboardInput) {
        match event {
            KeyboardInput {
                state: winit::event::ElementState::Pressed,
                virtual_keycode: Some(key),
                ..
            } => self.keydown(key),
            KeyboardInput {
                state: winit::event::ElementState::Released,
                virtual_keycode: Some(key),
                ..
            } => self.keyup(key),
            _ => {}
        }
    }

    fn keydown(&mut self, key: VirtualKeyCode) {
        match key {
            VirtualKeyCode::Left | VirtualKeyCode::A => self.first_port |= LEFT,
            VirtualKeyCode::C => self.first_port |= CREDIT,
            VirtualKeyCode::Right | VirtualKeyCode::D => self.first_port |= RIGHT,
            VirtualKeyCode::Space | VirtualKeyCode::F => self.first_port |= FIRE,
            VirtualKeyCode::Key1 => self.first_port |= P1_START,
            VirtualKeyCode::Key2 => self.first_port |= P2_START,
            _ => {}
        }
    }

    fn keyup(&mut self, key: VirtualKeyCode) {
        match key {
            VirtualKeyCode::Left | VirtualKeyCode::A => self.first_port &= !LEFT,
            VirtualKeyCode::C => self.first_port &= !CREDIT,
            VirtualKeyCode::Right | VirtualKeyCode::D => self.first_port &= !RIGHT,
            VirtualKeyCode::Space | VirtualKeyCode::F => self.first_port &= !FIRE,
            VirtualKeyCode::Key1 => self.first_port &= !P1_START,
            VirtualKeyCode::Key2 => self.first_port &= !P2_START,
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

fn main() {
    let mut machine = SpaceInvaders::new();

    let event_loop = EventLoop::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        // let scaled_size = LogicalSize::new(WIDTH as f64 * 3.0, HEIGHT as f64 * 3.0);
        WindowBuilder::new()
            .with_title("Space Invaders")
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap()
    };

    machine.update(pixels.get_frame());
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                    input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Escape), ..  },
                    ..
                },
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            },
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput{ input, ..},
                ..
            } => {
                machine.handle_event(input);
                machine.update(pixels.get_frame());
            },
            Event::RedrawRequested(_) => {
                machine.update(pixels.get_frame());
                pixels.render().unwrap();
            }
            _ => ()
        }

        window.request_redraw();
    });
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
