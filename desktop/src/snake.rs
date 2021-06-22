use crate::{MAP_WIDTH, MAP_HEIGHT};

#[derive(PartialOrd, PartialEq, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub struct Snake {
    pub body: Vec<[i32; 2]>,
    pub direction: Direction,
    pub speed: f32,
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            body: vec![[15, 15], [14, 15], [13, 15]],
            direction: Direction::Right,
            speed: 0.05,
        }
    }

    pub fn moving(&mut self, food: [i32; 2]) {
        let mut head = self.body[0];
        let map_width = MAP_WIDTH - 2;
        let map_height = MAP_HEIGHT - 2;
        match self.direction {
            Direction::Left => {
                head[0] -= 1;
                if head[0] < 0 {
                    head[0] = map_width as i32 - 1;
                }
            }
            Direction::Right => {
                head[0] = (head[0] + 1) % map_width as i32;
            }
            Direction::Up => {
                head[1] -= 1;
                if head[1] < 0 {
                    head[1] = map_height as i32 - 1;
                }
            }
            Direction::Down => {
                head[1] = (head[1] + 1) % map_height as i32;
            }
        }

        if head == food {
            self.body.insert(0, head);
        } else {
            let len = self.body.len();
            for i in 1..len {
                self.body[len - i] = self.body[len - i - 1];
            }
            self.body[0] = head;
        }
    }
}

impl Drop for Snake {
    fn drop(&mut self) {
        println!("蛇被释放了");
    }
}
