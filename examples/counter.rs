use std::io::Result;

use moonlight::{
    heartbeat::Heartbeat,
    input::{InputEvent, Key},
    BatchCmd,
};

/// A simple program that show a counter

/// A model can be more or less any type of data. It holds all the data for a
/// program, so often it's a struct.
#[derive(Clone)]
struct Model(i32);

/// Messages are events that we respond to in our Reducer function. This
/// particular one increment/decrement a counter
#[derive(Clone)]
enum Message {
    Increment,
    Decrement,
    Quit,
}

/// Reducer is called when messages are received. The idea is that you inspect
/// the message and retun a the updated model.
fn reducer(model: Model, message: Message) -> (Model, BatchCmd<Message>) {
    let model = match message {
        Message::Quit => {
            Heartbeat::stop(); // kill runtime
            Model(model.0)
        }
        Message::Increment => Model(model.0 + 1),
        Message::Decrement => Model(model.0 - 1),
    };
    (model, vec![])
}

/// View take data from the model and return a string which will be rendered
/// to the terminal.
fn view(model: &Model) -> String {
    String::from(model.0.to_string())
}

/// Input is called when stdin input are received. The idea is that you inspect
/// the event and returns an optional message.
fn input(event: InputEvent) -> Option<Message> {
    match event {
        InputEvent::Key(key) => match key {
            Key::Char('q') => Some(Message::Quit),
            Key::Char('k') => Some(Message::Increment),
            Key::Char('j') => Some(Message::Decrement),
            _ => None,
        },
        _ => None,
    }
}

fn main() -> Result<()> {
    let initialize = || (Model(0), None);
    moonlight::Runtime::new(reducer, initialize, input, view).run()
}
