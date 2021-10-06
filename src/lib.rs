#![allow(clippy::wildcard_imports)]

use flappy_ferris::{FlappyFerris, GameState};
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
    orders.stream(streams::interval(200, || Msg::Update));
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
                    GameState::Start => model.flappy_ferris.play_game(),
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
        playerbox(model),
        pipebox(model),
    ]
}

fn view_gamecontainer(model: &Model) -> Node<Msg> {
    div![
        attrs! {
            At::Id => "gamecontainer"
        },
        "GAME CONTAINER", // deleting this gets rid of the top bricks.. why?
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
        div![
            attrs! {
                At::Id => "flyarea",
            },
            view_ceiling(),
            view_player(),
            view_bigscore(),
            view_splash(),
            view_scoreboard(model),
        ]
    ]
}

fn view_ceiling() -> Node<Msg> {
    div![
        C!["animated"],
        attrs! {
            At::Id => "ceiling",
        }
    ]
}

fn view_player() -> Node<Msg> {
    div![
        C!["bird animated"],
        attrs! {
            At::Id => "player",
        }
    ]
}

fn view_bigscore() -> Node<Msg> {
    div![
        attrs! {
            At::Id => "bigscore"
        }
    ]
}

// Get ready part
fn view_splash() -> Node<Msg> {
    div![
        attrs! {At::Id => "splash"}
    ]
}

fn view_scoreboard(model: &Model) -> Node<Msg> {
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
            img![
                attrs! {
                    At::Src => "assets/replay.png",
                    At::Alt => "replay",
                }
            ]
        ],
    ]
}

fn view_land(model: &Model) -> Node<Msg> {
    div![
        C!["animated"],
        attrs! {
            At::Id => "land",
        }
    ]
}

fn view_footer() -> Node<Msg> {
    div![
        attrs! {At::Id => "footer"},
        a![
            "original game/concept/art by dong ngyuen",
            attrs! {At::Href => "https://www.dotgears.com/"},
        ],
        p![
            "recreated by ",
            strong!("sun hyuk ahn"),
        ],
        a![
            "view github project",
            attrs! {At::Href => "https://github.com/joycaleb9705"},
        ]
    ]
}

fn playerbox(model: &Model) -> Node<Msg> {
    div![
        C!["boudingbox"],
        attrs! {At::Id => "playerbox"}
    ]
}

fn pipebox(model: &Model) -> Node<Msg> {
    div![
        C!["boudingbox"],
        attrs! {At::Id => "pipebox"}
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
