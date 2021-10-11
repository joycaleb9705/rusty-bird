use rand::thread_rng;
use rand::Rng;
use serde::{Deserialize, Serialize};

const FLY_AREA: f64 = 420.0; // fly_area = top pipe height + bottom pipe height + pipe height (space in btwn)
const PIPE_HEIGHT: f64 = 90.0;
const PIPE_WIDTH: f64 = 50.0;
const PIPE_SHIFT: f64 = 2.0;
const JUMP: f64 = -4.5;
const GRAVITY: f64 = 0.25;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RustyBird {
    pub bird: Bird,
    pub pipe_manager: PipeManager,
    score: i32,
    high_score: i32,
    time: i32,
    game_state: GameState
}

impl RustyBird {
    pub fn new() -> RustyBird {
        RustyBird {
            bird: Bird::new(),
            pipe_manager: PipeManager::new(),
            score: 0,
            high_score: 0,
            time: 0,
            game_state: GameState::Start
        }
    }

    pub fn play_game(&mut self) {
        self.game_state = GameState::Playing;
    }

    pub fn reset_game(&mut self) {
        self.bird.reset();
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

    pub fn get_high_score(&self) -> i32 {
        self.high_score
    }
    
    pub fn update(&mut self) -> GameState {
        self.time += 1;
        self.bird.update();
        if self.bird.dead {
            self.game_state = GameState::Over;
            self.update_high_score();
            return self.game_state;
        }
        
        self.pipe_manager.update(self.time);

        if self.pipe_manager.pipes.is_empty() {
            return self.game_state;
        }

        let mut curr_pipe;
        match self.pipe_manager.get_first_unpassed() {
            Some(pipe) => curr_pipe = pipe,
            None => {return self.game_state;}
        }

        if self.bird.bird_box.right() > curr_pipe.upper_pipe.left() {
            if curr_pipe.intersects(&self.bird.bird_box) {
                self.game_state = GameState::Over;
                self.update_high_score();
                return self.game_state;
            }
        }

        if self.bird.bird_box.left() > curr_pipe.upper_pipe.right() {
            curr_pipe.passed = true;
            self.score += 1;
        }
        
        self.game_state
    }

    fn update_high_score(&mut self) {
        if self.score > self.high_score {
            self.high_score = self.score;
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum GameState {
    Start,
    Playing,
    Over,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Bird {
    pub bird_box: Box,
    pub speed: f64,
    pub dead: bool,
}

// Skip rotation
impl Bird {
    pub fn new() -> Bird {
        Bird {
            bird_box: Box::new(34.0, 24.0, 60.0, FLY_AREA / 2.0),
            speed: 0.0,
            dead: false,
        }
    }

    pub fn reset(&mut self) {
        self.bird_box.x = 60.0;
        self.bird_box.y = FLY_AREA / 2.0;
        self.speed = 0.0;
        self.dead = false;
    }

    pub fn jump(&mut self) {
        self.speed = JUMP;
    }

    pub fn update(&mut self) {
        self.speed += GRAVITY;
        self.bird_box.y += self.speed;
        if self.bird_box.top() < 0.0 {
            self.bird_box.y = 0.0;
        }
        if self.bird_box.bot() > FLY_AREA {
            self.bird_box.y = FLY_AREA;
            self.dead = true;
        }
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

    pub fn update(&mut self, time: i32) {
        if time > 0 && time % 80 == 0 {
            self.add_pipe();
        }
        for pipe in self.pipes.iter_mut() {
            pipe.shift();
        }
        self.remove_old();
    }

    fn add_pipe(&mut self) {
        let new_pipe = Pipe::new();
        self.pipes.push(new_pipe);
    }

    fn remove_old(&mut self) {
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
    pub upper_pipe: Box,
    pub lower_pipe: Box,
    pub passed: bool,
}

impl Pipe {
    pub fn new() -> Pipe {
        let pipe_x = 900.0;
        let upper_h = f64::from(thread_rng().gen_range(110, 221));
        let lower_h = FLY_AREA - PIPE_HEIGHT - upper_h;
        let upper_y = 0.0;
        let lower_y = upper_h + PIPE_HEIGHT;
        
        Pipe {
            upper_pipe: Box::new(PIPE_WIDTH, upper_h, pipe_x, upper_y),
            lower_pipe: Box::new(PIPE_WIDTH, lower_h, pipe_x, lower_y),
            passed: false,
        }
    }

    pub fn shift(&mut self) {
        self.upper_pipe.x -= PIPE_SHIFT;
        self.lower_pipe.x -= PIPE_SHIFT;
    }

    pub fn intersects(&self, other: &Box) -> bool {
        self.lower_pipe.intersects(other) || self.upper_pipe.intersects(other)
    }

    pub fn removable(&self) -> bool {
        self.lower_pipe.left() <= -80.00
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
        self.y + self.h
    }

    pub fn top(&self) -> f64 {
        self.y
    }

    pub fn intersects(&self, other: &Box) -> bool {
        self.right() >= other.left()
        && other.right() >= self.left()
        && self.bot() >= other.top()
        && other.bot() >= self.top()
    }
}

#[cfg(test)]
mod tests {

    use crate::rusty_bird::{RustyBird, GameState, Bird, Pipe};

    #[test]
    fn bird_no_jump() {
        let mut bird: Bird = Bird::new();
        let mut time: i32 = 0;

        while !bird.dead {
            bird.update();
            time += 1;
        }
        assert_eq!(time, 58);
    }

    #[test]
    fn test_only_jump() {
        let mut bird: Bird = Bird::new();
        // ferris.update();
        // println!("After 1 update: {:#?}", ferris);
        // ferris.update();
        // println!("After 2 update: {:#?}", ferris);
        // ferris.jump();
        // println!("After 1 jump: {:#?}", ferris);
        // ferris.update();
        // println!("After 3 update: {:#?}", ferris);    

        for i in 0..20 {
            bird.update();
            if i % 10 == 0 {
                bird.jump();
            }
        }
    }
}