#![allow(clippy::wildcard_imports)]

use rusty_bird::{RustyBird, GameState, Box};
use seed::{prelude::*, *};
use std::cmp;
// use tokio::time::{sleep, Duration};

mod rusty_bird;

const SPACE_KEY: &str = " ";

// ------ ------
//     Init
// ------ ------

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.stream(streams::window_event(Ev::KeyDown, |ev| Msg::Space(ev.unchecked_into())));
    orders.stream(streams::interval(15, || Msg::Update));
    let model = Model { 
        rusty_bird: rusty_bird::RustyBird::new() ,
    };
    model
}

// ------ ------
//     Model
// ------ ------

struct Model {
    rusty_bird: RustyBird,
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
enum Msg {
    Space(web_sys::KeyboardEvent),
    Update,
    Restart,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, order: &mut impl Orders<Msg>) {
    match msg {
        Msg::Space(ev) => {
            ev.prevent_default();
            if ev.key().as_str() == SPACE_KEY {
                match model.rusty_bird.get_game_state() {
                    GameState::Start => {
                        model.rusty_bird.play_game();
                        model.rusty_bird.bird.jump();
                    },
                    GameState::Over => {
                        model.rusty_bird.reset_game();
                    },
                    GameState::Playing => model.rusty_bird.bird.jump(),
                }
            }
        },
        Msg::Update => {
            match model.rusty_bird.get_game_state() {
                GameState::Playing => {
                    model.rusty_bird.update();
                },
                _ => (),
            }
        }
        Msg::Restart => model.rusty_bird.reset_game(),
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        view_gamecontainer(model),
        view_footer(),
    ]
}

fn view_gamecontainer(model: &Model) -> Node<Msg> {
    div![
        attrs! {
            At::Id => "gamecontainer"
        },
        view_gamescreen(model),
    ]
}

fn view_gamescreen(model: &Model) -> Node<Msg> {
    div![
        attrs! {
            At::Id => "gamescreen",
        },
        view_sky(model),
        view_land(model),
    ]
}

fn view_sky(model: &Model) -> Node<Msg> {
    div![
        C!["animated"],
        attrs! {
            At::Id => "sky",
        },
        IF!(matches!(model.rusty_bird.get_game_state(), GameState::Over) => 
            style! {
                St::AnimationPlayState => "paused"
            }
        ),
        view_flyarea(model),
    ]
}

fn view_flyarea(model: &Model) -> Node<Msg> {
    let game_state = model.rusty_bird.get_game_state();
    div![
        attrs! {
            At::Id => "flyarea",
        },
        view_ceiling(game_state),
        view_player(model),
        view_pipes(model),
        IF!(matches!(game_state, GameState::Playing) => 
            div![
                attrs! {
                    At::Id => "bigscore"
                },
                get_score(model.rusty_bird.get_score(), true),
            ]
        ),
        view_splash(game_state),
        view_scoreboard(model),
    ]
}

fn view_ceiling(game_state: GameState) -> Node<Msg> {
    div![
        C!["animated"],
        attrs! {
            At::Id => "ceiling",
        },
        IF!(matches!(game_state, GameState::Over) => 
            style! {
                St::AnimationPlayState => "paused"
            }
        )
    ]
}

fn view_player(model: &Model) -> Node<Msg> {
    div![
        C!["bird animated"],
        attrs! {
            At::Id => "player",
        },
        IF!(matches!(model.rusty_bird.get_game_state(), GameState::Playing) => 
            style! {
                St::Top => px(model.rusty_bird.bird.bird_box.y);
            }
        ),
        IF!(matches!(model.rusty_bird.get_game_state(), GameState::Over) =>
            style! {
                St::Opacity => 0;
            }
        )
    ]
}

fn view_pipes(model: &Model) -> Vec<Node<Msg>> {
    let mut pipe_vec = Vec::new();
    for pipe in &model.rusty_bird.pipe_manager.pipes {
        pipe_vec.push(
            div![
                view_upper(&pipe.upper_pipe),
                view_lower(&pipe.lower_pipe)
            ]
        )
    }
    pipe_vec
}

fn view_upper(upper_pipe: &Box) -> Node<Msg> {
    div![
        C!["pipe_upper"],
        style! {
            St::Height => px(upper_pipe.bot()),
            St::Left => px(upper_pipe.x),
        }
    ]
}

fn view_lower(lower_pipe: &Box) -> Node<Msg> {
    div![
        C!["pipe_lower"],
        style! {
            St::Height => px(420.0 - lower_pipe.top()),
            St::Left => px(lower_pipe.x),
        }
    ]
}

fn get_score(score: i32, big: bool) -> Vec<Node<Msg>> {
    let score_str = score.to_string();
    let score_chars = score_str.chars();
    let mut score = Vec::new();
    for c in score_chars {
        let src;
        if big {
            src = format! {
                "assets/font_big_{}.png",
                c
            };
        } else {
            src = format! {
                "assets/font_small_{}.png",
                c
            };
        }
        
        score.push(img![
            attrs! {
                At::Src => src
                At::Alt => c,
            }
        ]);
    }

    score
}

fn view_splash(game_state: GameState) -> Node<Msg> {
    div![
        attrs! {
            At::Id => "splash",
        },
        IF!(matches!(game_state, GameState::Start) => 
            style! {
                St::Opacity => 1,
            }
        )
    ]
}

fn view_scoreboard(model: &Model) -> Node<Msg> {
    let is_over = matches!(model.rusty_bird.get_game_state(), GameState::Over);
    let score = model.rusty_bird.get_score();
    div![
        attrs! {
            At::Id => "scoreboard"
        },
        div![
            attrs! {
                At::Id => "medal",
            },
            style! {
                St::Opacity => 1;
            },
            get_medal(score)
        ],
        div![
            attrs! {
                At::Id => "currentscore",
            },
            get_score(score, false),
        ],
        div![
            attrs! {
                At::Id => "highscore",
            },
            get_score(model.rusty_bird.get_high_score(), false),
            // get_score(model.rusty_bird.get_high_score(), false),
        ],
        div![
            attrs! {
                At::Id => "replay",
            },
            IF!(is_over => {
                style! {
                    St::Opacity => 1,
                }
            }),
            img![
                attrs! {
                    At::Src => "assets/replay.png",
                    At::Alt => "replay",
                }
            ],
            ev(Ev::Click, |_| Msg::Restart)
        ],
        IF!(is_over => style! {
            St::Display => "block",
            St::Opacity => 1,
            St::Transition => St::Opacity,
            St::TransitionDuration => "600ms",
            St::TransitionTimingFunction => "ease",
        })
    ]
}

fn get_medal(score: i32) -> Node<Msg> {
    let medal = match score {
        0..=9 => {return div![]},
        10..=19 => "bronze",
        20..=29 => "silver",
        30..=39 => "gold",
        _ => "platinum",
    };
    img![
        attrs! {
            At::Src => 
                format! {
                    "assets/medal_{}.png",
                    medal
                }
            At::Alt => medal,
        }
    ]
}


fn view_land(model: &Model) -> Node<Msg> {
    div![
        C!["animated"],
        attrs! {
            At::Id => "land",
        },
        IF!(matches!(model.rusty_bird.get_game_state(), GameState::Over) => 
            style! {
                St::AnimationPlayState => "paused"
            }
        )
    ]
}

fn view_footer() -> Node<Msg> {
    div![
        attrs! {
            At::Id => "footer"
        },
        a![
            "original game/concept/art by Dong Ngyuen",
            attrs! {
                At::Href => "https://www.dotgears.com/"
            },
        ],
        p![
            "recreated in Rust by ",
            strong!("Sun Hyuk Ahn"),
        ],
        a![
            "view Github project",
            attrs! {
                At::Href => "https://github.com/joycaleb9705"
            },
        ]
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
