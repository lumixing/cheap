use std::{env, fs};

use crate::emulator::*;
use macroquad::prelude::*;

mod emulator;

const DEBUG_HEIGHT: i32 = 128;
const PIXEL_SIZE: usize = 8;
const KEYS: [KeyCode; 16] = [
    KeyCode::X,
    KeyCode::Key1,
    KeyCode::Key2,
    KeyCode::Key3,
    KeyCode::Q,
    KeyCode::W,
    KeyCode::E,
    KeyCode::A,
    KeyCode::S,
    KeyCode::D,
    KeyCode::Z,
    KeyCode::C,
    KeyCode::Key4,
    KeyCode::R,
    KeyCode::F,
    KeyCode::V,
];

fn window_conf() -> Conf {
    Conf {
        window_title: "cheap".to_owned(),
        window_width: (SCREEN_WIDTH * PIXEL_SIZE) as i32,
        window_height: (SCREEN_HEIGHT * PIXEL_SIZE) as i32 + DEBUG_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let on_pixel_color = Color::from_hex(0x8bc8fe);
    let off_pixel_color = Color::from_hex(0x051b2c);

    let args: Vec<String> = env::args().collect();
    let input_path = args.get(1).expect("expected input path");
    let data = fs::read(&input_path).expect("could not read file");
    let rate: usize = args
        .get(2)
        .unwrap_or(&"120".to_owned())
        .parse()
        .unwrap_or(120);

    let mut emu = Emulator::new();
    emu.load(&data);

    let mut timer = 0.0;
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::Backspace) {
            emu.reset();
            emu.load(&data);
        }

        for (i, key) in KEYS.iter().enumerate() {
            if is_key_down(*key) {
                emu.keys[i] = true;
            }
            if is_key_released(*key) {
                emu.keys[i] = false;
            }
        }

        timer += get_frame_time();
        if timer > 1.0 / rate as f32 {
            emu.tick();
            emu.tick_timers();
            timer = 0.0;
        }

        // if is_key_pressed(KeyCode::Space) {
        // emu.tick();
        // }

        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                let pixel = emu.get_pixel(x, y);

                draw_rectangle(
                    x as f32 * PIXEL_SIZE as f32,
                    y as f32 * PIXEL_SIZE as f32,
                    PIXEL_SIZE as f32,
                    PIXEL_SIZE as f32,
                    if pixel {
                        on_pixel_color
                    } else {
                        off_pixel_color
                    },
                );
            }
        }

        let debug_offset = SCREEN_HEIGHT * PIXEL_SIZE;
        // let keys: Vec<usize> = emu
        //     .keys
        //     .iter()
        //     .enumerate()
        //     .filter(|(_, v)| **v)
        //     .map(|(i, _)| i)
        //     .collect();

        let debug_text_array = [
            format!("pc: {}", emu.pc),
            format!("op: {:#04x}", emu.op),
            format!("i: {}", emu.i_reg),
            format!("v_reg: {:?}", emu.v_reg),
            format!("stack: {:?}", emu.stack),
            format!(
                "keys: {:?}",
                // keys
                emu.keys
                    .iter()
                    // .enumerate()
                    // .collect::<bool>()
                    .map(|v| *v as u8)
                    .collect::<Vec<u8>>()
            ),
        ];

        for (i, text) in debug_text_array.iter().enumerate() {
            let text_offset = 12.0 * (i + 1) as f32;
            draw_text(text, 0.0, debug_offset as f32 + text_offset, 16.0, WHITE);
        }

        next_frame().await;
    }
}
