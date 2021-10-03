use rand::thread_rng;
use rand::Rng;
use serde::Deserialize;
use std::cmp;

const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 640.0;
const JUMP: f64 = 8.0;
const GRAVITY: f64 = 0.2;
const PIPE_SPEED: f64 = 8.0;

#[derive(Deserialize, Debug, Clone)]
pub struct FlappyFerris {
    pub ferris: Ferris,
    pipes: Vec<Pipe>,
    score: i32,
    time: i32,
    game_state: GameState
}

impl FlappyFerris {
    pub fn new() -> FlappyFerris {
        FlappyFerris {
            ferris: Ferris::new(),
            pipes: Vec::new(),
            score: 0,
            time: 0,
            game_state: GameState::New
        }
    }

    pub fn start_game(&mut self) {
        self.game_state = GameState::Playing;
    }

    pub fn reset_game(&mut self) {
        self.ferris.restart();
        self.pipes = Vec::new();
        self.score = 0;
        self.time = 0;
        self.game_state = GameState::New;
    }

    pub fn get_game_state(&self) -> GameState {
        self.game_state
    }
    
    // update pipe and ferris
    pub fn update(&mut self) -> GameState {
        self.ferris.update();
        if self.ferris.dead {
            self.game_state = GameState::Over;
            return self.game_state;
        }
        
        if self.time > 10 && self.time % 20 == 0 {
            self.add_pipes();
        }

        // Keep track of indices of pipes that need to be removed
        let mut to_remove = Vec::new();

        for i in 0..self.pipes.len() {
            let pipe: &mut Pipe = self.pipes.get_mut(i).unwrap();
            pipe.update();

            if !pipe.passed {
                if pipe.contains(self.ferris) {
                    self.game_state = GameState::Over;
                    return self.game_state;
                }

                if self.ferris.x > pipe.x + pipe.w {
                    pipe.passed = true;
                    if pipe.inverted {
                        self.score += 1;
                    }
                }
            }

            if pipe.x + pipe.w <= 0.0 {
                to_remove.push(i);       
            }
        }

        while !to_remove.is_empty() {
            self.pipes.remove(to_remove.pop().unwrap());
        }

        self.time += 1;
        self.game_state
    }

    pub fn add_pipes(&mut self) {
        // add og and inverted pipe to the pipes
        let bottom = Pipe::new();
        let top = Pipe::inverted_pipe(bottom.h);
        self.pipes.push(bottom);
        self.pipes.push(top);
    }
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum GameState {
    New,
    Playing,
    Over
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct Ferris {
    pub w: f64,
    pub h: f64,
    pub x: f64,
    pub y: f64,
    pub speed: f64,
    pub dead: bool,
}

impl Ferris {
    pub fn new() -> Ferris {
        Ferris {
            w: 25.0,
            h: 25.0,
            x: WIDTH / 2.0, // ferris centered at all time
            y: HEIGHT / 2.0,
            speed: 0.0,
            dead: false,
        }
    }

    pub fn restart(&mut self) {
        self.y = HEIGHT / 2.0;
        self.speed = 0.0;
        self.dead = false;
    }

    // TODO: Change logic! Curr problem: If we jump too many times, bird does not come down.
    pub fn jump(&mut self) {
        self.speed -= JUMP
    }

    pub fn update(&mut self) {
        self.y -= self.speed;
        if self.y > 640.0 {
            self.y = 640.0;
        }
        if self.y < 0.0 {
            self.dead = true;
        }
        self.speed += GRAVITY;
    }
}

// TODO: Bug: If bird stays at 640, game should end every 2 seconds because it should crash into the top (inverted) pipe
#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub struct Pipe {
    pub w: f64,
    pub h: f64,
    x: f64,
    pub inverted: bool,
    pub passed: bool,
}

impl Pipe {
    pub fn new() -> Pipe {
        Pipe {
            w: 50.0,
            h: f64::from(thread_rng().gen_range(100, 400)),
            x: 640.0,
            inverted: false,
            passed: false,
        }
    }

    pub fn inverted_pipe(h: f64) -> Pipe {
        Pipe {
            w: 50.0,
            h: 500.0 - h,
            x: 640.0,
            inverted: true,
            passed: false,
        }
    }

    pub fn update(&mut self) {
        self.x -= PIPE_SPEED;
    }

    pub fn contains(&self, f: Ferris) -> bool {
        // check four points
        // (x, y), (x + f.w, y), (x, y + f.h), (x + f.w, y + f.h)
        if self.contains_point(f.x, f.y) || self.contains_point(f.x + f.w, f.y) 
            || self.contains_point(f.x, f.y + f.h) || self.contains_point(f.x + f.w, f.y + f.h) {
                return true;
            }
        false
    }

    fn contains_point(&self, x: f64, y: f64) -> bool {
        if x >= self.x && x <= self.x + self.w {
            if self.inverted {
                return y >= HEIGHT - self.h;
            } else {
                return y <= self.h;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {

    use crate::flappy_ferris::{FlappyFerris, GameState, Ferris, Pipe};

    #[test]
    fn ferris_no_jump() {
        let mut ferris: Ferris = Ferris::new();
        let mut time: i32 = 0;

        while !ferris.dead {
            ferris.update();
            time += 1;
        }
        assert_eq!(time, 58);
    }

    #[test]
    fn ferris_jump() {
        let mut ferris: Ferris = Ferris::new();
        // ferris.update();
        // println!("After 1 update: {:#?}", ferris);
        // ferris.update();
        // println!("After 2 update: {:#?}", ferris);
        // ferris.jump();
        // println!("After 1 jump: {:#?}", ferris);
        // ferris.update();
        // println!("After 3 update: {:#?}", ferris);    

        for i in 0..20 {
            ferris.update();
            if i % 10 == 0 {
                ferris.jump();
            }
        }
    }
}