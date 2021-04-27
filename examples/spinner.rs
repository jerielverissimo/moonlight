use moonlight::{
    commands::map,
    components::spinner::{self, SpinnerType, TickMsg},
    Cmd,
};
use moonlight::{quit, Sub};
use termion::event::Key;

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
            return map(cmds);
        }
    }
    vec![]
}

fn view(model: &Model) -> String {
    let s = spinner::view(&model.spinner);
    format!("\n\n {} Loading forever...press q to quit\n\n", s)
}

fn input(event: Key) -> Option<Msg> {
    match event {
        Key::Char('q') => Some(Msg::Quit),
        _ => None,
    }
}

fn initialize() -> (Model, Option<Cmd<Msg>>) {
    let model = Model {
        spinner: spinner::Model::with(SpinnerType::MiniDot),
    };
    (model, Some(Box::new(|| Msg::from(spinner::tick()))))
}

fn main() {
    let subs: Vec<Sub<Model, Msg>> = Vec::new(); // type annotation to subs
    moonlight::program(initialize, update, view, input, subs);
}
