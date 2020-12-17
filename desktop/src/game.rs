use crate::snake::{Snake, Direction};
use crate::{MAP_WIDTH, MAP_HEIGHT};
use sdl2::keyboard::Keycode;
use std::option::Option::Some;

pub struct Game {
    snake: Snake,
    pub gfx: [u8; MAP_WIDTH * MAP_HEIGHT],
    move_time: f32,
    food: Option<[i32; 2]>,
    pub score: u32,
    pub over: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            snake: Snake::new(10, 10),
            gfx: [0; MAP_WIDTH * MAP_HEIGHT],
            move_time: 0.0,
            food: None,
            score: 0,
            over: false,
        }
    }

    pub fn reset(&mut self) {
        self.snake = Snake::new(10, 10);
        self.clear();
        self.over = false;
        self.food = None;
        self.score = 0;
    }

    fn clear(&mut self) {
        for i in 0..self.gfx.len() {
            if i < MAP_WIDTH || i > MAP_HEIGHT * MAP_WIDTH - MAP_WIDTH
                || i % MAP_WIDTH == 0 || i % MAP_WIDTH == MAP_WIDTH - 1 {
                self.gfx[i] = 1;
            } else {
                self.gfx[i] = 0;
            }
        }
    }

    pub fn frame(&mut self) {
        if self.over {
            return
        }
        if self.move_time < 1.0 {
            self.move_time += self.snake.speed;
        } else {
            self.move_time = 0.0;
            self.moving();
        }
    }

    fn draw(&mut self) {
        self.clear();
        let [x, y] = &self.snake.body[0];
        let i = x + 1 + (y + 1) * MAP_WIDTH as i32;
        self.gfx[i as usize] = 4;
        for [x, y] in self.snake.body[1..].iter() {
            let i = x + 1 + (y + 1) * MAP_WIDTH as i32;
            self.gfx[i as usize] = 2;
        }
        if let Some(food) = self.food {
            let i = food[0] + 1 + (food[1] + 1) * MAP_WIDTH as i32;
            self.gfx[i as usize] = 3;
        }
    }

    fn moving(&mut self) {
        match self.food {
            Some([x, y]) => {
                self.snake.moving([x, y]);
                let head = &self.snake.body[0];
                for i in 1..self.snake.body.len() {
                    if *head == self.snake.body[i] {
                        self.over = true;
                        break
                    }
                }
                if head[0] == x && head[1] == y {
                    self.score += 1;
                    self.place_food();
                    self.move_time = 0.0;
                }
            },
            None => {
                self.place_food();
            }
        }
        self.draw();
    }

    fn place_food(&mut self) {
        let [x, y] = [
            (rand::random::<u8>() % (MAP_WIDTH - 2) as u8) as i32,
            (rand::random::<u8>() % (MAP_HEIGHT - 2) as u8) as i32
        ];
        for [i, j] in self.snake.body.iter() {
            if *i == x && *j == y {
                return self.place_food()
            }
        }
        self.food = Some([x, y]);
    }

    fn turn(&mut self, direction: Direction) {
        if self.over {
            return
        }
        self.move_time = 0.0;
        self.snake.direction = direction;
        self.moving();
    }

    pub fn key_press(&mut self, key: Keycode) {
        let direction = &mut self.snake.direction;
        match key {
            Keycode::Right => {
                if *direction != Direction::Left {
                    self.turn(Direction::Right);
                }
            }
            Keycode::Up => {
                if *direction != Direction::Bottom {
                    self.turn(Direction::Top);
                }
            }
            Keycode::Down => {
                if *direction != Direction::Top {
                    self.turn(Direction::Bottom);
                }
            }
            Keycode::Left => {
                if *direction != Direction::Right {
                    self.turn(Direction::Left);
                }
            },
            _ => {}
        }
    }
}