use rand::thread_rng;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

const FLY_AREA: f64 = 420.0; // fly_area = top pipe height + bottom pipe height + pipe height (space in btwn)
const PIPE_HEIGHT: f64 = 90.0;
const PIPE_WIDTH: f64 = 50.0;
const JUMP: f64 = -4.5;
const GRAVITY: f64 = 0.25;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlappyFerris {
    pub ferris: Ferris,
    pub pipe_manager: PipeManager,
    score: i32,
    pub time: i32,
    game_state: GameState
}

impl FlappyFerris {
    pub fn new() -> FlappyFerris {
        FlappyFerris {
            ferris: Ferris::new(),
            pipe_manager: PipeManager::new(),
            score: 0,
            time: 0,
            game_state: GameState::Start
        }
    }

    pub fn play_game(&mut self) {
        self.game_state = GameState::Playing;
    }

    pub fn reset_game(&mut self) {
        self.ferris.reset();
        self.pipe_manager.reset();
        self.score = 0;
        self.time = 0;
        self.game_state = GameState::Start;
    }

    pub fn get_game_state(&self) -> GameState {
        self.game_state
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }
    
    // TODO: Update update
    pub fn update(&mut self) -> GameState {
        self.time += 1;
        self.ferris.update();
        if self.ferris.dead {
            self.game_state = GameState::Over;
            return self.game_state;
        }
        
        if self.time > 0 && self.time % 80 == 0 {
            self.pipe_manager.update(self.ferris.ferris_box.x);
        }

        if self.pipe_manager.pipes.is_empty() {
            return self.game_state;
        }

        // Update all pipe's x position by 2px

        let mut curr_pipe;
        match self.pipe_manager.get_first_unpassed() {
            Some(pipe) => curr_pipe = pipe,
            None => {return self.game_state;}
        }

        curr_pipe.upper_pipe.x -= 2.0;
        curr_pipe.lower_pipe.x -= 2.0;

        if self.ferris.ferris_box.right() > curr_pipe.upper_pipe.left() {
            if curr_pipe.intersects(&self.ferris.ferris_box) {
                self.game_state = GameState::Over;
                return self.game_state;
            }
        }

        if self.ferris.ferris_box.left() > curr_pipe.upper_pipe.right() {
            curr_pipe.passed = true;
            self.score += 1;
        }
        
        self.game_state
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum GameState {
    Start,
    Playing,
    Over,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Ferris {
    pub ferris_box: Box,
    pub speed: f64,
    pub dead: bool,
}

// Skip rotation
impl Ferris {
    pub fn new() -> Ferris {
        Ferris {
            ferris_box: Box::new(34.0, 24.0, 60.0, FLY_AREA / 2.0),
            speed: 0.0,
            dead: false,
        }
    }

    pub fn reset(&mut self) {
        self.ferris_box.x = 60.0;
        self.ferris_box.y = FLY_AREA / 2.0;
        self.speed = 0.0;
        self.dead = false;
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
        if self.ferris_box.y > FLY_AREA {
            self.ferris_box.y = FLY_AREA;
            self.dead = true;
        }

        // self.ferris_box.x += 60.0;
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PipeManager {
    pub pipes: Vec<Pipe>,
}

impl PipeManager {
    pub fn new() -> PipeManager {
        PipeManager{
            pipes: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.pipes = Vec::new();
    }

    pub fn update(&mut self, ferris_x: f64) {
        self.remove_old(ferris_x);
        self.add_pipe(ferris_x);
    }

    fn add_pipe(&mut self, ferris_x: f64) {
        let new_pipe = Pipe::new(ferris_x);
        self.pipes.push(new_pipe);
    }

    fn remove_old(&mut self, ferris_x: f64) {
        self.pipes.retain(|&pipe| !pipe.removable());
    }

    pub fn get_first_unpassed(&mut self) -> Option<&mut Pipe> {
        for pipe in self.pipes.iter_mut() {
            if !pipe.passed {
                return Some(pipe);
            }
        };
        None
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub struct Pipe {
    pub lower_pipe: Box,
    pub upper_pipe: Box,
    pub passed: bool,
}

impl Pipe {
    pub fn new(ferris_x: f64) -> Pipe {
        // let pipe_x = ferris_x + 200.0;
        let pipe_x = 900.0;
        let lower_h = f64::from(thread_rng().gen_range(110, 221));
        let upper_h = FLY_AREA - PIPE_HEIGHT - lower_h;
        let lower_y = 0.0;
        let upper_y = lower_y + lower_h + PIPE_HEIGHT;
        
        Pipe {
            lower_pipe: Box::new(PIPE_WIDTH, lower_h, pipe_x, lower_y),
            upper_pipe: Box::new(PIPE_WIDTH, upper_h, pipe_x, upper_y),
            passed: false,
        }
    }

    pub fn intersects(&self, other: &Box) -> bool {
        self.lower_pipe.intersects(other, false) || self.upper_pipe.intersects(other, true)
    }

    pub fn removable(&self) -> bool {
        // self.lower_pipe.left() + 100.0 < ferris_x
        self.lower_pipe.left() <= -100.00
    }
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
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

    pub fn left(&self) -> f64 {
        self.x
    }

    pub fn right(&self) -> f64 {
        self.x + self.w
    }

    pub fn bot(&self) -> f64 {
        self.y
    }

    pub fn top(&self) -> f64 {
        self.y + self.h
    }

    pub fn intersects(&self, other: &Box, inverted: bool) -> bool {
        if !inverted {
            self.left() <= other.right()
            && other.left() <= self.right()
            && self.bot() <= other.top()
            && other.bot() <= self.top()
        } else {
            self.left() <= other.right()
            && other.left() <= self.right()
            && self.bot() <= other.top()
            && other.bot() <= self.top()
        }
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
    fn test_only_jump() {
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