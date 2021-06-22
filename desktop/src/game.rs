use crate::snake::{Snake, Direction};
use crate::{MAP_WIDTH, MAP_HEIGHT};
use sdl2::keyboard::Keycode;
use std::option::Option::Some;
use std::cmp::Ordering;
use crate::a_star::a_star_search;

pub struct Game {
    snake: Snake,
    pub gfx: [u8; MAP_WIDTH * MAP_HEIGHT],
    move_time: f32,
    food: Option<[i32; 2]>,
    pub score: u32,
    pub over: bool,
    plans: Vec<[i32; 2]>,
    auto_plan: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            snake: Snake::new(),
            gfx: [0; MAP_WIDTH * MAP_HEIGHT],
            move_time: 0.0,
            food: None,
            score: 0,
            over: false,
            plans: Vec::new(),
            auto_plan: false,
        }
    }

    pub fn reset(&mut self) {
        self.snake = Snake::new();
        self.clear();
        self.over = false;
        self.food = None;
        self.score = 0;
        self.plans.clear();
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
        let head = self.snake.body[0];
        if let Some(next_pos) = self.plans.pop() {
            match (next_pos[0].cmp(&head[0]), next_pos[1].cmp(&head[1])) {
                (Ordering::Greater, _) => {
                    self.snake.direction = Direction::Right
                }
                (Ordering::Less, _) => {
                    self.snake.direction = Direction::Left
                }
                (_, Ordering::Greater) => {
                    self.snake.direction = Direction::Down
                }
                (_, Ordering::Less) => {
                    self.snake.direction = Direction::Up
                }
                _ => {}
            }
        }
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
                    self.snake.speed += 0.001;
                    self.place_food();
                    self.move_time = 0.0;
                }
            },
            None => {
                self.place_food();
            }
        }
        // let now = std::time::Instant::now();
        self.plan();
        // println!("{:?}", now.elapsed());
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
        // self.plan();
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
                if *direction != Direction::Down {
                    self.turn(Direction::Up);
                }
            }
            Keycode::Down => {
                if *direction != Direction::Up {
                    self.turn(Direction::Down);
                }
            }
            Keycode::Left => {
                if *direction != Direction::Right {
                    self.turn(Direction::Left);
                }
            },
            Keycode::A => {
                self.auto_plan = !self.auto_plan;
                if !self.auto_plan {
                    self.plans.clear();
                }
                self.plan();
            },
            Keycode::W => self.snake.speed += 0.1,
            _ => {}
        }
    }

    fn plan(&mut self) {
        if self.auto_plan {
            self.plans = a_star_search(&self.snake.body[1..], self.snake.body[0], self.food.unwrap());
        }
    }
}