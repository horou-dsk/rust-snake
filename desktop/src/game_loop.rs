use sdl2::render::{WindowCanvas, TextureQuery};
use sdl2::pixels::Color;
use sdl2::Sdl;
use chrono::Local;
use std::env;
use std::path::Path;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::thread::sleep;
use sdl2::rect::Rect;
use crate::render_text::{Text, draw_text};
use sdl2::ttf::FontStyle;
use crate::game::Game;
use crate::colors;

const MS_PER_UPDATE: f64 = 100000000.0 / 6.0;

const SCREEN_WIDTH: u32 = 450;
const SCREEN_HEIGHT: u32 = 320;

pub struct GameLoop {
    sdl_context: Sdl,
    game: Game,
    canvas: WindowCanvas,
}

impl GameLoop {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("snake", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();
        Self {
            game: Game::new(),
            canvas,
            sdl_context,
        }
    }

    pub fn run(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let mut next_game_tick = Local::now().timestamp_nanos() as f64;
        let canvas = &mut self.canvas;
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
        let mut text = Text::new("得分：0", Color::RGB(0xFF, 0xFF, 0xFF), 16, FontStyle::NORMAL);
        text.set_position(330, 5);
        let mut over_text = Text::new("GAME OVER", Color::RGB(0xFF, 0, 0), 32, FontStyle::BOLD);
        let game = &mut self.game;
        'running: loop {
            over_text.set_center(SCREEN_WIDTH, SCREEN_HEIGHT);
            next_game_tick += MS_PER_UPDATE;
            let sleep_time = next_game_tick - Local::now().timestamp_nanos() as f64;

            canvas.set_draw_color(Color::RGB(16, 29, 43));
            canvas.clear();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    Event::KeyDown { keycode: Some(key), .. } => {
                        if key == Keycode::R {
                            game.reset();
                        } else {
                            game.key_press(key);
                        }
                    },
                    _ => {
                        // println!("{:?}", e);
                    }
                }
            }

            game.frame();

            text.text = format!("得分：{}", game.score);
            draw_text(&mut text, &ttf_context, canvas);

            let gfx = &game.gfx;
            for i in 0..gfx.len() {
                if gfx[i] != 0 {
                    canvas.set_draw_color([colors::WALL, colors::SNAKE, colors::FOOD, colors::HEAD][gfx[i] as usize - 1]);
                    let rect = rect!((i % 32 * 10) as i32, (i / 32 * 10) as i32, 10, 10);
                    canvas.fill_rect(rect).unwrap();
                    canvas.draw_rect(rect).unwrap();
                }
            }

            if game.over {
                draw_text(&mut over_text, &ttf_context, canvas);
            }
            canvas.present();
            if sleep_time > 0.0 {
                sleep(Duration::new(0, sleep_time as u32));
            }
        }
    }
}
