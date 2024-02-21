use crate::emulator::*;
use macroquad::prelude::*;

mod emulator;

const DEBUG_HEIGHT: i32 = 128;
const PIXEL_SIZE: usize = 8;

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

    let mut emu = Emulator::new();

    const DATA: [u8; 132] = [
        // Offset 0x00000000 to 0x00000083
        0x00, 0xE0, 0xA2, 0x2A, 0x60, 0x0C, 0x61, 0x08, 0xD0, 0x1F, 0x70, 0x09, 0xA2, 0x39, 0xD0,
        0x1F, 0xA2, 0x48, 0x70, 0x08, 0xD0, 0x1F, 0x70, 0x04, 0xA2, 0x57, 0xD0, 0x1F, 0x70, 0x08,
        0xA2, 0x66, 0xD0, 0x1F, 0x70, 0x08, 0xA2, 0x75, 0xD0, 0x1F, 0x12, 0x28, 0xFF, 0x00, 0xFF,
        0x00, 0x3C, 0x00, 0x3C, 0x00, 0x3C, 0x00, 0x3C, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0x00, 0xFF,
        0x00, 0x38, 0x00, 0x3F, 0x00, 0x3F, 0x00, 0x38, 0x00, 0xFF, 0x00, 0xFF, 0x80, 0x00, 0xE0,
        0x00, 0xE0, 0x00, 0x80, 0x00, 0x80, 0x00, 0xE0, 0x00, 0xE0, 0x00, 0x80, 0xF8, 0x00, 0xFC,
        0x00, 0x3E, 0x00, 0x3F, 0x00, 0x3B, 0x00, 0x39, 0x00, 0xF8, 0x00, 0xF8, 0x03, 0x00, 0x07,
        0x00, 0x0F, 0x00, 0xBF, 0x00, 0xFB, 0x00, 0xF3, 0x00, 0xE3, 0x00, 0x43, 0xE0, 0x00, 0xE0,
        0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0xE0, 0x00, 0xE0,
    ];
    emu.load(&DATA);

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::Space) {
            emu.tick();
        }

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

        let debug_text_array = [
            format!("pc: {}", emu.pc),
            format!("op: {:#04x}", emu.op),
            format!("i: {}", emu.i_reg),
            format!("v_reg: {:?}", emu.v_reg),
        ];

        for (i, text) in debug_text_array.iter().enumerate() {
            let text_offset = 12.0 * (i + 1) as f32;
            draw_text(text, 0.0, debug_offset as f32 + text_offset, 16.0, WHITE);
        }

        next_frame().await;
    }
}
