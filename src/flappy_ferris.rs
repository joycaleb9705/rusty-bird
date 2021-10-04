use rand::thread_rng;
use rand::Rng;
use serde::Deserialize;
use std::cmp;

// const WIDTH: f64 = 640.0;
const FLY_AREA: f64 = 360.0; // fly_area = top pipe height + bottom pipe height + pipe height (space in btwn)
const PIPE_HEIGHT: f64 = 90.0;
const PIPE_WIDTH: f64 = 50.0;
const JUMP: f64 = -4.5;
const GRAVITY: f64 = 0.25;

#[derive(Deserialize, Debug, Clone)]
pub struct FlappyFerris {
    pub ferris: Ferris,
    pipe_manager: PipeManager,
    score: i32,
    time: i32,
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
    
    // TODO: Update update
    pub fn update(&mut self) -> GameState {
        self.ferris.update();
        if self.ferris.dead {
            self.game_state = GameState::Over;
            return self.game_state;
        }
        
        if self.time > 0 && self.time % 1600 == 0 {
            self.pipe_manager.update(self.ferris.ferris_box.x);
        }

        if self.pipe_manager.pipes.is_empty() {
            return self.game_state;
        }

        // TODO: unsafe; should do a None check
        let mut curr_pipe = self.pipe_manager.get_first_unpassed().unwrap();

        if self.ferris.ferris_box.right() > curr_pipe.top_pipe.left() {
            if curr_pipe.intersects(&self.ferris.ferris_box) {
                self.game_state = GameState::Over;
                return self.game_state;
            }
        }

        if self.ferris.ferris_box.left() > curr_pipe.top_pipe.right() {
            curr_pipe.passed = true;
            self.score += 1;
        }
        
        self.time += 1;
        self.game_state
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
    pub dead: bool,
}

// Skip rotation
impl Ferris {
    pub fn new() -> Ferris {
        Ferris {
            ferris_box: Box::new(35.0, 25.0, 60.0, 180.0),
            speed: 0.0,
            dead: false,
        }
    }

    pub fn reset(&mut self) {
        self.ferris_box.x = 60.0;
        self.ferris_box.y = 180.0;
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
            self.dead = true;
        }
        if self.ferris_box.y > 360.0 {
            self.ferris_box.y = 360.0;
        }

        self.ferris_box.x += 60.0;
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
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
        self.pipes.retain(|&pipe| !pipe.removable(ferris_x));
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

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub struct Pipe {
    pub bot_pipe: Box,
    pub top_pipe: Box,
    pub passed: bool,
}

impl Pipe {
    // New pipe's x is curr ferris's x + 480 (1.6 secs)
    pub fn new(ferris_x: f64) -> Pipe {
        let pipe_x = ferris_x + 480.0;
        let bot_height = f64::from(thread_rng().gen_range(80, 191));
        let top_height = FLY_AREA - PIPE_HEIGHT - bot_height;
        let bot_y = 0.0;
        let top_y = bot_y + bot_height + PIPE_HEIGHT;
        
        Pipe {
            bot_pipe: Box::new(PIPE_WIDTH, bot_height, pipe_x, bot_y),
            top_pipe: Box::new(PIPE_WIDTH, top_height, pipe_x, top_y),
            passed: false,
        }
    }

    pub fn intersects(&self, other: &Box) -> bool {
        self.top_pipe.intersects(other) || self.bot_pipe.intersects(other)
    }

    pub fn removable(&self, ferris_x: f64) -> bool {
        self.bot_pipe.x + 100.0 < ferris_x
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