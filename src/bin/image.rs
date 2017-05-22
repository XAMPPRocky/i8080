extern crate i8080;
extern crate piston_window;
extern crate image;

use i8080::*;
use image::RgbaImage;
use piston_window::*;

const HEIGHT: u32 = 224;
const WIDTH: u32 = 256;

fn main() {
    let mut buffer = RgbaImage::new(WIDTH, HEIGHT);
    let mut machine = SpaceInvaders::new();

    let (mut window, mut texture) = {
        update(&mut machine, &mut buffer);

        let mut window: PistonWindow =
            WindowSettings::new("SpaceInvaders", [HEIGHT, WIDTH])
            .exit_on_esc(true)
            .opengl(OpenGL::V3_2)
            .build()
            .unwrap();

        let texture = Texture::from_image(
            &mut window.factory,
            &buffer,
            &TextureSettings::new()
        ).unwrap();
        (window, texture)
    };

    while let Some(e) = window.next() {
        update(&mut machine, &mut buffer);

        machine.handle_event(&e);

        texture.update(&mut window.encoder, &buffer).unwrap();
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            image(&texture, [[0., 2./HEIGHT as f64, -1.], [2./WIDTH as f64, 0., -1.]], g);
        });
    }
}

fn update(machine: &mut SpaceInvaders, buffer: &mut RgbaImage) {
    machine.emulate();

    for (i, byte) in machine.framebuffer().iter().enumerate() {
        const SHIFT_END: u8 = 7;

        let y = i * 8 / (WIDTH as usize + 1);
        for shift in 0..SHIFT_END + 1 {
            let x = ((i * 8) % (WIDTH as usize)) + shift as usize;

            let pixel = if (byte >> shift) & 1 == 0 {
                [0, 0, 0, 255]
            } else {
                [255; 4]
            };

            buffer.put_pixel(x as u32, y as u32, image::Rgba(pixel));
        }
    }
}

