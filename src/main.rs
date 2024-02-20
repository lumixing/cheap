use macroquad::prelude::*;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const DEBUG_HEIGHT: i32 = 128;
const PIXEL_SIZE: usize = 8;

struct Emulator {
    screen: Screen,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            screen: Screen::new(),
        }
    }
}

struct Screen(Vec<bool>);

impl Screen {
    pub fn new() -> Self {
        Self(vec![false; SCREEN_WIDTH * SCREEN_HEIGHT])
    }

    pub fn get(&self, x: u8, y: u8) -> Option<bool> {
        let i = self.linearize(x, y);

        if self.index_out_of_bounds(i) {
            return None;
        }

        Some(self.0[i])
    }

    pub fn set(&mut self, x: u8, y: u8, value: bool) -> bool {
        let i = self.linearize(x, y);

        if self.index_out_of_bounds(i) {
            return false;
        }

        self.0[i] = value;
        true
    }

    fn index_out_of_bounds(&self, i: usize) -> bool {
        i >= self.0.len()
    }

    fn linearize(&self, x: u8, y: u8) -> usize {
        x as usize + y as usize * SCREEN_WIDTH
    }
}

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
    let mut emu = Emulator::new();

    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            let pixel = if x % 2 == 0 { y % 2 == 0 } else { y % 2 == 1 };
            emu.screen.set(x as u8, y as u8, pixel);
        }
    }

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        for (i, pixel) in emu.screen.0.iter().enumerate() {
            let x = i % SCREEN_WIDTH;
            let y = i / SCREEN_WIDTH;

            draw_rectangle(
                x as f32 * PIXEL_SIZE as f32,
                y as f32 * PIXEL_SIZE as f32,
                PIXEL_SIZE as f32,
                PIXEL_SIZE as f32,
                if *pixel { RED } else { BLUE },
            )
        }

        let debug_offset = SCREEN_HEIGHT * PIXEL_SIZE;

        draw_text(
            &format!("fps: {}", get_fps()),
            0.0,
            debug_offset as f32 + 12.0,
            16.0,
            WHITE,
        );

        next_frame().await;
    }
}
