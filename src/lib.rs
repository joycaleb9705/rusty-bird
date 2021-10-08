#![allow(clippy::wildcard_imports)]

use flappy_ferris::{FlappyFerris, GameState, Box};
use seed::{prelude::*, *};

mod flappy_ferris;

const SPACE_KEY: &str = "Space";

// ------ ------
//     Init
// ------ ------

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    // TODO: Curr Ver: Space handles all the events
    // TODO: Update Ver: Separate into different MSG
    orders.stream(streams::window_event(Ev::KeyDown, |ev| Msg::Space(ev.unchecked_into())));
    orders.stream(streams::interval(15, || Msg::Update));
    let model = Model { 
        flappy_ferris: flappy_ferris::FlappyFerris::new() 
    };
    model
}

// ------ ------
//     Model
// ------ ------

struct Model {
    flappy_ferris: FlappyFerris,
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
enum Msg {
    Space(web_sys::KeyboardEvent),
    Start,
    Jump,
    Update,
    Restart,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Space(ev) => {
            ev.prevent_default();
            if ev.key().as_str() == "Enter" {
                match model.flappy_ferris.get_game_state() {
                    GameState::Start => {
                        model.flappy_ferris.play_game();
                        model.flappy_ferris.ferris.jump();
                    },
                    GameState::Over => model.flappy_ferris.reset_game(),
                    GameState::Playing => model.flappy_ferris.ferris.jump(),
                }
            }
        },
        Msg::Start => model.flappy_ferris.play_game(),
        Msg::Jump => model.flappy_ferris.ferris.jump(),
        Msg::Update => {
            match model.flappy_ferris.get_game_state() {
                GameState::Playing => {model.flappy_ferris.update();},
                _ => (),
            }
        }
        Msg::Restart => model.flappy_ferris.reset_game(),
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
        // playerbox(model),
        // pipebox(model),
    ]
}

fn view_gamecontainer(model: &Model) -> Node<Msg> {
    div![
        attrs! {
            At::Id => "gamecontainer"
        },
        "NEED THIS FOR CEILING", // deleting this gets rid of the top bricks.. why?
        view_gamescreen(model),
    ]
}

fn view_gamescreen(model: &Model) -> Node<Msg> {
    div![
        attrs! {
            At::Id => "gamescreen",
        },
        view_sky(model),
        view_land(),
    ]
}

fn view_sky(model: &Model) -> Node<Msg> {
    div![
        C!["animated"],
        attrs! {
            At::Id => "sky",
        },
        view_flyarea(model),
    ]
}

fn view_flyarea(model: &Model) -> Node<Msg> {
    let game_state = model.flappy_ferris.get_game_state();
    div![
        attrs! {
            At::Id => "flyarea",
        },
        view_ceiling(),
        view_player(model),
        div![
            format! {
                "TIME: {}, FERRIS X: {}, FERRIS Y: {}",
                model.flappy_ferris.time,
                model.flappy_ferris.ferris.ferris_box.x,
                model.flappy_ferris.ferris.ferris_box.y,
            }
        ],
        view_pipes(model),
        IF!(matches!(game_state, GameState::Playing) => 
            view_bigscore(model.flappy_ferris.get_score())
        ),
        view_splash(game_state),
        view_scoreboard(game_state),
    ]
}

fn view_ceiling() -> Node<Msg> {
    div![
        C!["animated"],
        attrs! {
            At::Id => "ceiling",
        },
    ]
}

fn view_player(model: &Model) -> Node<Msg> {
    div![
        C!["bird animated"],
        attrs! {
            At::Id => "player",
        },
        IF!(matches!(model.flappy_ferris.get_game_state(), GameState::Playing) => 
            style! {
                St::Top => px(model.flappy_ferris.ferris.ferris_box.y);
            }
        )
    ]
}

fn view_pipes(model: &Model) -> Node<Msg> {
    if !model.flappy_ferris.pipe_manager.pipes.is_empty() {
        div![
            C!["pipe animated"],
            format! {
                "PIPE_X: {}, UPPER_PIPE_BOT: {}, LOWER_PIPE_TOP: {}",
                model.flappy_ferris.pipe_manager.pipes.get(0).unwrap().lower_pipe.x,
                model.flappy_ferris.pipe_manager.pipes.get(0).unwrap().upper_pipe.bot(),
                model.flappy_ferris.pipe_manager.pipes.get(0).unwrap().lower_pipe.top(),
            },
            view_upper(&model.flappy_ferris.pipe_manager.pipes.get(0).unwrap().upper_pipe),
            view_lower(&model.flappy_ferris.pipe_manager.pipes.get(0).unwrap().lower_pipe)
        ]
    } else {
        div![
            "NO PIPE"
        ]
    }
        
}

// fn view_pipes(model: &Model) -> Vec<Node<Msg>> {
//     let mut pipe_vec = Vec::new();
//     for pipe in &model.flappy_ferris.pipe_manager.pipes {
//         pipe_vec.push(
//             div![
//                 C!["pipe animated"],
//                 format! {
//                     "PIPE_X: {}",
//                     &pipe.lower_pipe.x
//                 },
//                 view_upper(&pipe.upper_pipe),
//                 view_lower(&pipe.lower_pipe)
//             ]
//         )
//     }
//     pipe_vec
// }

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

// Not centered, the front digit is centered but not the second digit
fn view_bigscore(score: i32) -> Node<Msg> {
    let score_str = score.to_string();
    let score_chars = score_str.chars();
    let mut big_score = Vec::new();
    for c in score_chars {
        let src = format!{
            "assets/font_big_{}.png",
            c
        };
        big_score.push(img![
            attrs! {
                At::Src => src
                At::Alt => c,
            }
        ]);
    }

    div![
        attrs! {
            At::Id => "bigscore"
        },
        big_score,
    ]
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

fn view_scoreboard(game_state: GameState) -> Node<Msg> {
    let is_over = matches!(game_state, GameState::Over);
    div![
        attrs! {
            At::Id => "scoreboard"
        },
        div![
            attrs! {
                At::Id => "medal",
            },
        ],
        div![
            attrs! {
                At::Id => "currentscore",
            },
        ],
        div![
            attrs! {
                At::Id => "highscore",
            },
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
            St::Display => "block"
        })
    ]
}

fn view_land() -> Node<Msg> {
    div![
        C!["animated"],
        attrs! {
            At::Id => "land",
        }
    ]
}

fn view_footer() -> Node<Msg> {
    div![
        attrs! {
            At::Id => "footer"
        },
        a![
            "original game/concept/art by dong ngyuen",
            attrs! {
                At::Href => "https://www.dotgears.com/"
            },
        ],
        p![
            "recreated by ",
            strong!("sun hyuk ahn"),
        ],
        a![
            "view github project",
            attrs! {
                At::Href => "https://github.com/joycaleb9705"
            },
        ]
    ]
}

// fn playerbox(model: &Model) -> Node<Msg> {
//     div![
//         C!["boudingbox"],
//         attrs! {
//             At::Id => "playerbox"
//         },
//     ]
// }

// fn pipebox(model: &Model) -> Node<Msg> {
//     div![
//         C!["boudingbox"],
//         attrs! {
//             At::Id => "pipebox"
//         },
//     ]
// }

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
