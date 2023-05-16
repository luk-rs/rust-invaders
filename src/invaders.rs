use std::cmp::max;
use std::time::Duration;
use rusty_time::Timer;
use crate::{NUM_COLS, NUM_ROWS};
use crate::frame::{Drawable, Frame};

pub struct Invader {
    x: usize,
    y: usize
}

pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: i32, // 1 -> right, -1 <- left
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if x % 3 == 0 && y % 2 == 0 && y < NUM_ROWS / 2 {
                    army.push(Invader { x, y })
                }
            }
        }
        Self {
            army,
            move_timer: Timer::from_millis(2000),
            direction: 1
        }
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);

        if self.move_timer.ready {
            self.move_timer.reset();
            let mut downwards = false;
            if self.direction == -1 {
                let at_left = self.army.iter().any(|invader| invader.x == 0);
                if at_left {
                    self.direction = 1;
                    downwards = true;
                }
            } else {
                let at_right = self.army.iter().any(|invader| invader.x == NUM_COLS - 1);
                if at_right {
                    self.direction = -1;
                    downwards = true;
                }
            }

            if downwards {
                let speed_up = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(speed_up as u64);
            }

            for invader in self.army.iter_mut() {
                if downwards {
                    invader.y = (invader.y as i32 + 1) as usize;
                } else {
                    invader.x = (invader.x as i32 + self.direction) as usize;
                }
            }

            return true;
        }

        false
    }

    pub fn exterminated(&self) -> bool {
        self.army.is_empty()
    }

    pub fn reached_ship(&self) -> bool {
        self.army.iter().any(|invader| invader.y == NUM_ROWS - 1)
    }

    pub fn kill_invader_at(&mut self, x:usize, y:usize) -> bool {
        let killed = self.army.iter().position(|invader|invader.x == x && invader.y == y);
        match killed {
            Some(idx) => {
                self.army.remove(idx);
                true
            },
            _ => false
        }
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            let stretching = (self.move_timer.time_left.as_secs_f32() / self.move_timer.duration.as_secs_f32()) > 0.5;
            frame[invader.x][invader.y] = if stretching { "%" } else { "&" };
        }
    }
}
