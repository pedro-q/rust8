use std::fs::File;
use std::io::Read;
use std::time::Duration;
use std::thread;
use std::env;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;

mod cpu;

// The CHIP-8 fontset
static CHIP8_FONTSET: [u8; 80] =
[
    0xF0, 0x90, 0x90, 0x90, 0xF0, //0
    0x20, 0x60, 0x20, 0x20, 0x70, //1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, //2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, //3
    0x90, 0x90, 0xF0, 0x10, 0x10, //4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, //5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, //6
    0xF0, 0x10, 0x20, 0x40, 0x40, //7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, //8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, //9
    0xF0, 0x90, 0xF0, 0x90, 0x90, //A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, //B
    0xF0, 0x80, 0x80, 0x80, 0xF0, //C
    0xE0, 0x90, 0x90, 0x90, 0xE0, //D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, //E
    0xF0, 0x80, 0xF0, 0x80, 0x80  //F
];

fn read_rom(romfile: &String) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(romfile)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    return Ok(data);
}

fn main() -> Result<(), String>  {

    // APP INIT ------

    // Get console arguments
    // args[1]: The rom filename
    let args: Vec<String> = env::args().collect();

    // Helpers and SDL2 graphics initialization
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Rust 8", 640, 320)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let mut pixels: [u8; (64 * 32) * 3] = [0; (64 * 32) * 3];
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, 64, 32)
        .map_err(|e| e.to_string())?;
    canvas.clear();
    canvas.copy(&texture, None, Some(Rect::new(0, 0, 640, 320)))?;
    canvas.present();

    // SDL2 Keyboard event pump
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Initialize the cpu system
    let mut cpu = cpu::CPU::initialize();

    // CHIP-8 INIT ------

    // Read the ROM
    let rom = match read_rom(&args[1]) {
        Ok(value) => value,
        Err(_error) => Vec::new() // TODO: Not let this silently fail!
    };

    // Load the chip8 font
    for i in 0..80 {
        cpu.memory[i] = CHIP8_FONTSET[i];
    }

    // Load ROM into the memory
    for (x, &val) in rom.iter().enumerate() {
        cpu.memory[0x200 + x] = val;
    }

    // MAIN LOOP ----

    'mainloop: loop {
        // TODO: Maybe make this block of code cleaner?
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::X), ..} => {
                    cpu.key[0] = 1
                },
                Event::KeyDown { keycode: Some(Keycode::Num1), ..} => {
                    cpu.key[1] = 1
                },
                Event::KeyDown { keycode: Some(Keycode::Num2), ..} => {
                    cpu.key[2] = 1
                },
                Event::KeyDown { keycode: Some(Keycode::Num3), ..} => {
                    cpu.key[3] = 1
                },
                Event::KeyDown { keycode: Some(Keycode::Q), ..} => {
                    cpu.key[4] = 1
                },
                Event::KeyDown { keycode: Some(Keycode::W), ..} => {
                    cpu.key[5] = 1
                },
                Event::KeyDown { keycode: Some(Keycode::E), ..} => {
                    cpu.key[6] = 1
                },
                Event::KeyDown { keycode: Some(Keycode::A), ..} => {
                    cpu.key[7] = 1
                },
                Event::KeyDown { keycode: Some(Keycode::S), ..} => {
                    cpu.key[8] = 1
                },
                Event::KeyDown { keycode: Some(Keycode::D), ..} => {
                    cpu.key[9] = 1
                },
                Event::KeyDown { keycode: Some(Keycode::Z), ..} => {
                    cpu.key[10] = 1
                },
                Event::KeyDown { keycode: Some(Keycode::C), ..} => {
                    cpu.key[11] = 1
                },
                Event::KeyDown { keycode: Some(Keycode::Num4), ..} => {
                    cpu.key[12] = 1
                },
                Event::KeyDown { keycode: Some(Keycode::R), ..} => {
                    cpu.key[13] = 1
                },
                Event::KeyDown { keycode: Some(Keycode::F), ..} => {
                    cpu.key[14] = 1
                },
                Event::KeyDown { keycode: Some(Keycode::V), ..} => {
                    cpu.key[15] = 1
                },
                Event::KeyUp { keycode: Some(Keycode::X), ..} => {
                    cpu.key[0] = 0
                },
                Event::KeyUp { keycode: Some(Keycode::Num1), ..} => {
                    cpu.key[1] = 0
                },
                Event::KeyUp { keycode: Some(Keycode::Num2), ..} => {
                    cpu.key[2] = 0
                },
                Event::KeyUp { keycode: Some(Keycode::Num3), ..} => {
                    cpu.key[3] = 0
                },
                Event::KeyUp { keycode: Some(Keycode::Q), ..} => {
                    cpu.key[4] = 0
                },
                Event::KeyUp { keycode: Some(Keycode::W), ..} => {
                    cpu.key[5] = 0
                },
                Event::KeyUp { keycode: Some(Keycode::E), ..} => {
                    cpu.key[6] = 0
                },
                Event::KeyUp { keycode: Some(Keycode::A), ..} => {
                    cpu.key[7] = 0
                },
                Event::KeyUp { keycode: Some(Keycode::S), ..} => {
                    cpu.key[8] = 0
                },
                Event::KeyUp { keycode: Some(Keycode::D), ..} => {
                    cpu.key[9] = 0
                },
                Event::KeyUp { keycode: Some(Keycode::Z), ..} => {
                    cpu.key[10] = 0
                },
                Event::KeyUp { keycode: Some(Keycode::C), ..} => {
                    cpu.key[11] = 0
                },
                Event::KeyUp { keycode: Some(Keycode::Num4), ..} => {
                    cpu.key[12] = 0
                },
                Event::KeyUp { keycode: Some(Keycode::R), ..} => {
                    cpu.key[13] = 0
                },
                Event::KeyUp { keycode: Some(Keycode::F), ..} => {
                    cpu.key[14] = 0
                },
                Event::KeyUp { keycode: Some(Keycode::V), ..} => {
                    cpu.key[15] = 0
                },
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } |
                Event::Quit { .. } => break 'mainloop,
                _ => {}
            }
        }

        // Fetch opcode
        cpu.fetch_opcode();

        // Decrease delay timer
        if cpu.delay_timer > 0 {
            cpu.delay_timer -= 1;
        }

        // If sound timer reaches 0 let's beep
        if cpu.sound_timer > 0 { 
            if cpu.sound_timer == 1 {
                // TODO: Some real sound but looks difficult in SDL2 :(
                println!("BEEEP");
            }
            cpu.sound_timer -= 1
        }

        // Draw the screen
        if cpu.draw {
            for (y, &val) in cpu.gfx.iter().enumerate() {
                let offset = y * 3;
                let bitval = 255 * val;
                pixels[offset] = bitval;
                pixels[offset + 1] = bitval;
                pixels[offset + 2] = bitval;
            }
            texture.update(None, &pixels, 64 * 3).map_err(|e| e.to_string())?;
            canvas.clear();
            canvas.copy(&texture, None, Some(Rect::new(0, 0, 640, 320)))?;
            canvas.present();
            cpu.draw = false;
        }

        // Just a flag to break the loop if an unknown instruction is found
        if cpu.breakexe {
            break
        }

        // Let's sleep for a while instead of fetching the next instruction
        thread::sleep(Duration::from_millis(2))
    }

    Ok(())

}
