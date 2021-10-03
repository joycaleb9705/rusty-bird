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
    // TODO: Curr Ver: Msg::Update checks the GameState so that update occures only during GameState::Playing
    // TODO: Update Ver: Msg::Update gets sent only during GameState::Playing instead of all times
    orders.stream(streams::interval(100, || Msg::Update));
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
                    GameState::New => model.flappy_ferris.start_game(),
                    GameState::Over => model.flappy_ferris.reset_game(),
                    GameState::Playing => model.flappy_ferris.ferris.jump(),
                }
            }
        },
        Msg::Start => model.flappy_ferris.start_game(),
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
fn view(model: &Model) -> Node<Msg> {
    // if model.flappy_ferris.get_game_state() == GameState::New {
    //     view_new(model)
    // } else if model.flappy_ferris.get_game_state() == GameState::Playing {
    //     view_playing(model)
    // } else {
    //     view_dead(model)
    // }

    // div![
    //     C!["gamecontainer"],
    //     div![
    //         C!["gamescreen"],
    //         div![ // <div id="sky" class="animated">
    //             C!["sky"],

    //         ],
    //         div![
    //             C!["land"]
    //         ],
    //         div![
    //             C!["debug"]
    //         ]
    //     ]
    // ],
    div![
        view_gamecontainer(model),
        view_footer(),
    ]
}

fn view_gamecontainer(model: &Model) -> Node<Msg> {
    div![
        C!["gamecontainer"],
        "GAME CONTAINER",
        div![
            C!["gamescreen"],
            view_sky(model),
            div![ // <div id="land" class="animated">
                C!["land"],
            ],
        ]
    ]
}

fn view_sky(model: &Model) -> Node<Msg> {
    div![C!["sky"], // <div id="sky" class="animated">
        div![C!["flyarea"],
            // <div id="ceiling" class="animated"></div>
            view_ceiling(),
            // <!-- This is the flying and pipe area container -->

            // <div id="player" class="bird animated"></div>
            view_player(),
            // <div id="bigscore"></div>

            // <div id="splash"></div>
            view_scoreboard(model),
        ]
    ]
}

fn view_ceiling() -> Node<Msg> {
    div![
        C!["celing"]
    ]
}

fn view_player() -> Node<Msg> {
    div![
        C!["player"],
    ]
}

fn view_scoreboard(model: &Model) -> Node<Msg> {
    div![
        C!["scoreboard"],
        div![
            C!["meadl"],
        ],
        div![
            C!["currentscore"],
        ],
        div![
            C!["highscore"],
        ],
        div![
            C!["replay"],
            // img![
            //     attrs! {At::src => "assets/replay.png", At::alt => "replay"},
            // ]
        ],
    ]
}

fn view_footer() -> Node<Msg> {
    div![
        C!["footer"],
        a![
            "original game/concept/art by dong ngyuen",
            attrs! {At::Href => "https://www.dotgears.com/"},
        ],
        br![],
        "recreated by ",
        strong!("sun hyuk ahn"),
        br![],
        a![
            "view github project",
            attrs! {At::Href => "https://github.com/joycaleb9705"},
        ]
    ]
}

fn view_new(model: &Model) -> Node<Msg> {
    section![
        h1![C!["new-game", "is-medium"],
            "New Game"
            ],
        button!["Play Game", ev(Ev::Click, |_| Msg::Start),]
    ]
}

fn view_playing(model: &Model) -> Node<Msg> {
    section![
        h1![C!["play-game", "is-medium"],
            "Playing Game"
        ],
        div![
            C!["Flappy Ferris"],
            "Flappy Ferris",
            "Dead: ", model.flappy_ferris.ferris.dead as i32,
            "Y val: ", model.flappy_ferris.ferris.y as i32,
            "Speed: ", model.flappy_ferris.ferris.speed as i32,
        ],
        button!["Update", ev(Ev::Click, |_| Msg::Update),],
        // ev(Ev::KeyUp, |_| Msg::Jump),
    ]
}

fn view_dead(model: &Model) -> Node<Msg> {
    section![
        h1![C!["end-game", "is-medium"],
            "Game Over"
        ],
        button!["Restart Game", ev(Ev::Click, |_| Msg::Restart),]
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
