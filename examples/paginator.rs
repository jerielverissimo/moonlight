use moonlight::{
    components::paginator::{self, PaginatorType},
    render::{exit_fullscreen, fullscreen},
    BatchCmd, Cmd,
};
use moonlight::{quit, Sub};
use termion::event::Key;

/// A simple paginator demo with dots and arabic style

/// A model can be more or less any type of data. It holds all the data for a
/// program, so often it's a struct.
#[derive(Clone)]
struct Model {
    paginator: paginator::Model,
}

/// Messages are events that we respond to in our Update function. This
/// particular one pass input data to paginator model
enum Msg {
    Input(Key),
    Quit,
}

/// Update is called when messages are received. The idea is that you inspect
/// the message and update the model.
fn update(msg: Msg, model: &mut Model) -> BatchCmd<Msg> {
    match msg {
        Msg::Quit => quit(),
        Msg::Input(key) => {
            match key {
                Key::Char('a') => model.paginator.paginator_type(PaginatorType::Arabic),
                Key::Char('d') => model.paginator.paginator_type(PaginatorType::Dots),
                _ => {}
            }
            paginator::input::<Msg>(&mut model.paginator, key);
        }
    }
    vec![]
}

/// View take data from the model and return a string which will be rendered
/// to the terminal.
fn view(model: &Model) -> String {
    paginator::view(&model.paginator)
}

/// Input is called when stdin input are received. The idea is that you inspect
/// the event and returns an optional message.
fn input(event: Key) -> Option<Msg> {
    match event {
        Key::Char('q') => Some(Msg::Quit),
        _ => Some(Msg::Input(event)),
    }
}

fn initialize() -> (Model, Option<Cmd<Msg>>) {
    let mut paginator = paginator::Model::new();
    paginator.set_total_pages(4);
    (Model { paginator }, None)
}

fn main() {
    fullscreen();
    let subs: Vec<Sub<Model, Msg>> = Vec::new(); // type annotation to subs
    moonlight::program(initialize, update, view, input, subs);
    exit_fullscreen();
}
