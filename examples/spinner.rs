use std::io::Result;

use moonlight::{
    commands,
    components::spinner::{self, SpinnerType, TickMsg},
    heartbeat::Heartbeat,
    input::{InputEvent, Key},
    Cmd,
};

/// A simple program demonstrating the spinner component from the Moonlight
/// component library.

#[derive(Clone)]
struct Model {
    spinner: spinner::Model,
}

#[derive(Clone)]
enum Msg {
    SpinnerTick(TickMsg),
    Quit,
}

impl From<TickMsg> for Msg {
    fn from(m: TickMsg) -> Self {
        Self::SpinnerTick(m)
    }
}

fn reducer(model: Model, msg: Msg) -> (Model, Vec<impl Fn() -> Msg>) {
    let mut model = Model { ..model };

    match msg {
        Msg::Quit => Heartbeat::stop(),
        Msg::SpinnerTick(msg) => {
            let cmds = spinner::reducer(&mut model.spinner, msg);
            return (model, commands::map_batch(cmds));
        }
    }
    (model, vec![])
}

fn view(model: &Model) -> String {
    let s = spinner::view(&model.spinner);
    format!("\n\n {} Loading forever...press q to quit\n\n", s)
}

fn input(event: InputEvent) -> Option<Msg> {
    match event {
        InputEvent::Key(key) => match key {
            Key::Char('q') => Some(Msg::Quit),
            _ => None,
        },
        _ => None,
    }
}

fn initialize() -> (Model, Option<Cmd<Msg>>) {
    let model = Model {
        spinner: spinner::Model::with(SpinnerType::MiniDot),
    };
    let ignition = || Msg::from(spinner::tick());
    (model, Some(Box::new(ignition)))
}

fn main() -> Result<()> {
    moonlight::Runtime::new(reducer, initialize, input, view).run()
}
