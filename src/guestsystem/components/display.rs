use std::{thread, time::Duration};

use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window, Sdl, VideoSubsystem};

use super::cpu::Cpu;

const COLOR_OFF: Color = Color::BLACK;
const COLOR_ON: Color = Color::WHITE;
const WIDTH: u8 = 64;
const HEIGHT: u8 = 32;
const SIZE_MULTIPLIER: u8 = 10;
const REFRESH_FPS: f64 = 160.0;

pub struct DisplayScreen<'a> {
    context: &'a Sdl,
    canvas: Canvas<Window>,
    pixels: [[bool; HEIGHT as usize]; WIDTH as usize],
}

impl<'a> DisplayScreen<'a> {
    pub fn new(context: &Sdl) -> DisplayScreen {
        let video_subsystem: VideoSubsystem = context.video().unwrap();
        let window: Window = video_subsystem
            .window(
                "RCHIP-8",
                WIDTH as u32 * SIZE_MULTIPLIER as u32,
                HEIGHT as u32 * SIZE_MULTIPLIER as u32,
            )
            .position_centered()
            .build()
            .unwrap();
        let canvas: Canvas<Window> = window.into_canvas().build().unwrap();
        DisplayScreen {
            context,
            canvas,
            pixels: [[false; HEIGHT as usize]; WIDTH as usize],
        }
    }

    pub fn clear_screen(&mut self) {
        for mut col in self.pixels {
            col.fill(false);
        }
        self.canvas.set_draw_color(COLOR_OFF);
        self.canvas.clear();
        thread::sleep(Duration::from_secs_f64(1.0 / REFRESH_FPS));
        self.canvas.present();
    }

    pub fn display(&mut self, x_coord: u8, y_coord: u8, sprite: &[u8], cpu: &mut Cpu) {
        let mut y: u8 = y_coord % HEIGHT;
        cpu.set_flag_register(0);

        for data in sprite {
            let mut x: u8 = x_coord % WIDTH;
            let mut byte: u8 = data.clone();
            for _ in 0..8 {
                let bit: bool = byte & 0x80 == 0x80;
                let pixel: bool = self.pixels[x as usize][y as usize];
                if bit && pixel {
                    self.erase_pixel(x, y);
                    cpu.set_flag_register(1);
                } else if bit && !pixel {
                    self.draw_pixel(x, y);
                }
                if x == WIDTH - 1 {
                    break;
                }
                x += 1;
                byte <<= 1;
            }
            if y == HEIGHT - 1 {
                break;
            }
            y += 1;
        }
    }

    fn draw_pixel(&mut self, x: u8, y: u8) {
        self.pixels[x as usize][y as usize] = true;
        self.canvas.set_draw_color(COLOR_ON);
        let x_canv: i32 = (x as i32) * (SIZE_MULTIPLIER as i32);
        let y_canv: i32 = (y as i32) * (SIZE_MULTIPLIER as i32);
        let px_size: u32 = SIZE_MULTIPLIER as u32;
        self.canvas
            .fill_rect(Rect::new(x_canv, y_canv, px_size, px_size))
            .expect("Error while drawing pixel on display.");
        thread::sleep(Duration::from_secs_f64(1.0 / REFRESH_FPS));
        self.canvas.present();
    }

    fn erase_pixel(&mut self, x: u8, y: u8) {
        self.pixels[x as usize][y as usize] = false;
        self.canvas.set_draw_color(COLOR_OFF);
        let x_canv: i32 = (x as i32) * (SIZE_MULTIPLIER as i32);
        let y_canv: i32 = (y as i32) * (SIZE_MULTIPLIER as i32);
        let px_size: u32 = SIZE_MULTIPLIER as u32;
        self.canvas
            .fill_rect(Rect::new(x_canv, y_canv, px_size, px_size))
            .expect("Error while erasing pixel on display.");
        thread::sleep(Duration::from_secs_f64(1.0 / REFRESH_FPS));
        self.canvas.present();
    }
}
