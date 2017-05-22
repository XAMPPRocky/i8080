extern crate i8080;
use i8080::{Cpu, Machine};
use std::io::Read;
use std::fs::File;

struct Facade;

impl Machine for Facade {
    fn input(&mut self, _: u8) -> u8  {0}
    fn output(&mut self, _: u8, _: u8) {}
}

fn main() {

    if !cfg!(feature = "cpudiag") {
        println!("Needs to be run with the cpudiag flag");
        ::std::process::exit(1);
    }

    let buffer = {
        let mut buf = Vec::new();
        let mut file = File::open("./TEST.COM").unwrap();
        file.read_to_end(&mut buf).unwrap();
        buf
    };

    let mut cpu = Cpu::new();
    let mut breakpoint = false;
    let adr = 0x100u16;
    cpu.load_into_rom(&buffer, 0x100);
    cpu.pc = adr.into();


    loop {

        if false {
            breakpoint = true;
        }

        if breakpoint {
            println!("{:?}", cpu);
            cpu.print_opcode();
            ::std::io::stdin().read_line(&mut String::new()).unwrap();
        }

        cpu.print_opcode();
        cpu.emulate(&mut Facade);
    }
}

