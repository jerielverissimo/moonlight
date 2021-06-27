use std::io::Result;

use moonlight::{
    components::paginator::{self, PaginatorType},
    heartbeat::Heartbeat,
    input::{InputEvent, Key},
    BatchCmd, Cmd,
};

/// A simple paginator demo with dots and arabic style

/// A model can be more or less any type of data. It holds all the data for a
/// program, so often it's a struct.
#[derive(Clone)]
struct Model {
    paginator: paginator::Model,
}

/// Messages are events that we respond to in our Update function. This
/// particular one pass input data to paginator model
#[derive(Clone)]
enum Msg {
    Input(Key),
    Quit,
}

/// Reducer is called when messages are received. The idea is that you inspect
/// the message and update the model.
fn reducer(model: &Model, msg: &Msg) -> (Model, BatchCmd<Msg>) {
    let mut model = Model {
        paginator: model.paginator.clone(),
    };

    match msg {
        Msg::Quit => Heartbeat::stop(),
        Msg::Input(key) => {
            match key {
                Key::Char('a') => model.paginator.paginator_type(PaginatorType::Arabic),
                Key::Char('d') => model.paginator.paginator_type(PaginatorType::Dots),
                _ => {}
            }
            paginator::input::<Msg>(&mut model.paginator, *key);
        }
    }
    (model, vec![])
}

/// View take data from the model and return a string which will be rendered
/// to the terminal.
fn view(model: &Model) -> String {
    paginator::view(&model.paginator)
}

/// Input is called when stdin input are received. The idea is that you inspect
/// the event and returns an optional message.
fn input(event: InputEvent) -> Option<Msg> {
    match event {
        InputEvent::Key(event) => match event {
            Key::Char('q') => Some(Msg::Quit),
            key => Some(Msg::Input(key)),
        },
        _ => None,
    }
}

fn initialize() -> (Model, Option<Cmd<Msg>>) {
    let mut paginator = paginator::Model::new();
    paginator.set_total_pages(4);
    (Model { paginator }, None)
}

fn main() -> Result<()> {
    moonlight::Runtime::new(reducer, initialize, input, view)
        .with_fullscreen()
        .run()
}
