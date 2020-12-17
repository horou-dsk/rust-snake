use sdl2::pixels::Color;
use sdl2::ttf::{FontStyle, Sdl2TtfContext};
use std::env;
use std::path::Path;
use sdl2::render::{WindowCanvas, TextureCreator, TextureQuery};
use sdl2::rect::Rect;

pub fn draw_text(text: &mut Text, ttf_context: &Sdl2TtfContext, canvas: &mut WindowCanvas) {
    let dir = env::var("windir").unwrap() + "\\Fonts\\PINGFANG REGULAR.TTF";
    let path = Path::new(&dir);
    let mut font = ttf_context.load_font(path, text.font_size).unwrap();
    font.set_style(text.font_style);
    let surface = font
        .render(text.text.as_str())
        .blended(text.color)
        .map_err(|e| e.to_string()).unwrap();

    let texture_creator = canvas.texture_creator();

    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string()).unwrap();
    let TextureQuery { width, height, .. } = texture.query();
    text.width = width;
    text.height = height;
    let target = rect!(text.x, text.y, width, height);

    canvas.copy(&texture, None, target).unwrap();
}

#[derive(Clone)]
pub struct Text {
    pub text: String,
    pub color: Color,
    pub width: u32,
    pub height: u32,
    pub font_style: FontStyle,
    pub font_size: u16,
    pub x: i32,
    pub y: i32,
}

impl Text {
    pub fn new<S: Into<String>>(text: S, color: Color, font_size: u16, font_style: sdl2::ttf::FontStyle) -> Self {
        Self {
            text: text.into(),
            color,
            font_size,
            font_style,
            width: 0,
            height: 0,
            x: 0,
            y: 0,
        }
    }

    pub fn set_position(&mut self, x: i32, y: i32) -> &mut Text {
        self.x = x;
        self.y = y;
        self
    }

    pub fn set_center(&mut self, width: u32, height: u32) -> &mut Text {
        self.x = (width / 2 - self.width / 2) as i32;
        self.y = (height / 2 - self.height / 2) as i32;
        self
    }
}
