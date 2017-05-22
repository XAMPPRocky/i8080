#[macro_use] extern crate clap;
extern crate i8080;

use i8080::*;

fn main() {
    let _ = clap_app!(tokei =>
        (version: crate_version!())
        (author: "Aaron P. <theaaronepower@gmail.com>")
        (about: crate_description!())
        (@arg debugger: -d --debugger "Run debugger")
    ).get_matches();


    let mut invaders = SpaceInvaders::new();

    loop {
        invaders.emulate();
    }
}
