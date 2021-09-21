use rand::thread_rng;

const WIDTH: i32 = 640;
const HEIGHT: i32 = 640;
const JUMP: i32 = 8;
const GRAVITY: f62 = 0.2;
const PIPE_SPEED: i32 = 4;

pub struct FlappyFerris {
    ferris: Ferris,
    pipes: Vec<Pipe>,
    score: i32,
    time: i32,
    game_state: GameState
}

impl FlappyFerris {
    FlappyFerris {
        ferris: Ferris::new(),
        pipes: Vec::new(),
        score: 0,
        time: 0,
        game_state: GameState::New
    }

    pub fn is_game_over(&self) -> bool {
        self.game_state == GameState::Over
    }
    
    // update pipe and ferris
    pub fn update() -> GameState {
        self.ferris.update();
        if self.ferris.y < 0 {
            self.game_state = GameState::Over;
            return self.game_state;
        }
        
        if time > 10 && time % 10 == 0 {
            self.add_pipes();
        }

        // Keep track of indices of pipes that need to be removed
        let mut to_remove = Vec::new();

        for (i, pipe) in self.pipes.iter().enumerate() {
            pipe.update();

            // Only check pipes not yet passed
            if !pipe.passed {
                // If pipe contains ferris, end game
                if pipe.contains(self.ferris) {
                    self.game_state = GameState::Over;
                    return self.game_state;
                }

                // If not yet passed and f.x > p.x + p.w
                if self.ferris.x > pipe.x + pipe.w {
                    pipe.passed = true;
                    // Only when passing inverted pipe bc og pipe is always ahead of inverted 
                    // and ferris need to pass both
                    if pipe.inverted {
                        self.score += 1;
                    }
                }
            }

            // If condition met, include the index of curr pipe to the remove list
            if pipe.x + pipe.w <= 0 {
                to_remove.push(i);
            }
            
        }

        while !to_remove.is_empty() {
            self.pipes.remove(to_remove.pop());
        }

        time += 1;
        self.game_state
    }
    
    fn add_pipes() {
        // add og and inverted pipe to the pipes
        let bottom = Pipe::new();
        let top = Pipe::inverted_pipe(bottom.h);
        vec.push(bottom);
        vec.push(top);
    }
}

enum GameState {
    New,
    Playing,
    Over
}

pub struct Ferris {
    w: i32,
    h: i32,
    x: i32,
    y: i32,
    speed: i32,
    pub dead: bool,
}

impl Ferris {
    pub fn new() -> Ferris {
        Ferris {
            w: 25,
            h: 25,
            x: WIDTH / 2, // ferris centered at all time
            y: HEIGHT / 2,
            speed: 0,
            dead: false,
        }
    }

    pub fn restart(&mut self) -> Ferris {
        self.y = HEIGHT / 2;
        self.speed = 0;
        self.dead = false;
    }

    pub fn jump(&mut self) {
        self.speed -= JUMP
    }

    pub fn update(&mut self) {
        self.y -= self.speed;
        if self.y < 0 {
            self.dead = true;
        }
        self.speed += GRAVITY;
    }
}

pub struct Pipe {
    pub w: i32,
    pub h: i32,
    x: i32,
    pub inverted: bool,
    pub passed: bool,
}

impl Pipe {
    pub fn new() -> Pipe {
        Pipe {
            w: 50,
            h: thread_rng().gen_range(100, 400),
            x: 640,
            inverted: false,
            passed: false,
        }
    }

    pub fn inverted_pipe(h: i32) -> Pipe {
        Pipe {
            w: 50,
            h: 400 - h,
            x: 640,
            inverted: true,
            passed: false,
        }
    }

    pub fn update(&mut self) {
        self.x -= PIPE;
    }

    pub fn contains(f: Ferris) -> bool {
        // check four points
        // (x, y), (x + f.w, y), (x, y + f.h), (x + f.w, y + f.h)
        if contains_point(f.x, f.y) || contains_point(f.x + f.w, f.y) 
            || contains_point(f.x, f.y + f.h) || contains_point(f.x + f.w, f.y + f.h) {
                return true;
            }
        false
    })

    fn contains_point(x: i32, y: i32) -> bool {
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