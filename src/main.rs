#![allow(dead_code)]
mod apu;
mod cartridge;
mod hardware;
mod mapper;
mod ppu;
mod render;

use cartridge::rom::NESRom;
use hardware::cpu::Cpu;
use render::frame::Frame;
use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum, EventPump};

fn handle_user_input(_cpu: &mut Cpu, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => std::process::exit(0),
            _ => (),
        }
    }
}
fn main() {
    // init sdl2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Tile viewer", (256.0 * 3.0) as u32, (240.0 * 3.0) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(3.0, 3.0).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 256, 240)
        .unwrap();

    let cartridge = Box::new(NESRom::from_file("priv/pacman.nes").unwrap());
    let mut cpu = Cpu::new(cartridge);
    cpu.reset();

    // run the game cycle
    let mut frame = Frame::new();
    let mut i = 0;
    loop {
        i += 1;
        cpu.execute_next_opcode();

        if i % 30_000 == 0 {
            render::render(&cpu.bus.ppu, &mut frame);
            texture.update(None, &frame.data, 256 * 3).unwrap();

            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
        }
        handle_user_input(&mut cpu, &mut event_pump);
    }
}
