use rand::thread_rng;
use rand::Rng;
use serde::Deserialize;
use std::cmp;

const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 640.0;
const JUMP: f64 = -4.5;
const GRAVITY: f64 = 0.25;
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
            game_state: GameState::Start
        }
    }

    pub fn play_game(&mut self) {
        self.game_state = GameState::Playing;
    }

    pub fn reset_game(&mut self) {
        self.ferris.restart();
        self.pipes = Vec::new();
        self.score = 0;
        self.time = 0;
        self.game_state = GameState::Start;
    }

    pub fn get_game_state(&self) -> GameState {
        self.game_state
    }
    
    // TODO: Update update
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

    // TODO: Don't need this
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
    Start,
    Playing,
    Over,
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct Ferris {
    pub ferris_box: Box,
    pub speed: f64,
}

// Skip rotation
impl Ferris {
    pub fn new() -> Ferris {
        Ferris {
            ferris_box: Box::new(35.0, 25.0, 60.0, 180.0),
            speed: 0.0,
        }
    }

    pub fn restart(&mut self) {
        self.ferris_box.x = 60.0;
        self.ferris_box.y = 180.0;
        self.speed = 0.0;
    }

    pub fn jump(&mut self) {
        self.speed = JUMP;
    }

    pub fn update(&mut self) {
        self.speed += GRAVITY;
        self.ferris_box.y += self.speed;
        if self.ferris_box.y < 0.0 {
            self.ferris_box.y = 0.0;
        }
        if self.ferris_box.y > 360.0 {
            self.ferris_box.y = 360.0;
        }

        self.ferris_box.x += 60.0;
    }
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub struct Pipe {
    pub upper_box: Box,
    pub lower_box: Box,
    pub passed: bool,
}

impl Pipe {
    pub fn new() -> Pipe {
        Pipe {
            // TODO: Calculate height with randomness and padding
            upper_box: Box::new(90.0, 50.0, 0.0, 0.0),
            lower_box: Box::new(90.0, 50.0, 0.0, 0.0),
            passed: false,
        }
    }

    pub fn intersects(&self, other: &Box) -> bool {
        self.upper_box.intersects(other) || self.lower_box.intersects(other)
    }
}


#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub struct Box {
    pub w: f64,
    pub h: f64,
    pub x: f64,
    pub y: f64,
}

impl Box {
    pub fn new(w: f64, h: f64, x: f64, y: f64) -> Box {
        Box {
            w,
            h,
            x,
            y,
        }
    }

    pub fn intersects(&self, other: &Box) -> bool {
        self.x <= other.x + other.w && self.y <= other.y + other.h && other.x <= self.x + self.x && other.y <= self.y + self.y
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