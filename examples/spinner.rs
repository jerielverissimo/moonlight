use std::io::Result;

use moonlight::{
    commands,
    components::spinner::{self, SpinnerType, TickMsg},
    input::{InputEvent, Key},
    Cmd,
};
use moonlight::{quit, Sub};

/// A simple program demonstrating the spinner component from the Moonlight
/// component library.

#[derive(Clone)]
struct Model {
    spinner: spinner::Model,
}

enum Msg {
    SpinnerTick(TickMsg),
    Quit,
}

impl From<TickMsg> for Msg {
    fn from(m: TickMsg) -> Self {
        Self::SpinnerTick(m)
    }
}

fn update(msg: Msg, model: &mut Model) -> Vec<impl Fn() -> Msg> {
    match msg {
        Msg::Quit => quit(),
        Msg::SpinnerTick(msg) => {
            let cmds = spinner::update(msg, &mut model.spinner);
            return commands::map_batch(cmds);
        }
    }
    vec![]
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
    (model, Some(Box::new(|| Msg::from(spinner::tick()))))
}

fn main() -> Result<()> {
    let subs: Vec<Sub<Model, Msg>> = Vec::new(); // type annotation to subs
    moonlight::program(initialize, update, view, input, subs)?;
    Ok(())
}
