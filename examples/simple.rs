use std::{io::Result, thread, time::Duration};

use moonlight::{heartbeat::Heartbeat, input::InputEvent, BatchCmd};

/// A simple program that counts down from 5 and then exits.

/// A model can be more or less any type of data. It holds all the data for a
/// program, so often it's a struct.
#[derive(Clone)]
struct Model(i32);

/// Messages are events that we respond to in our Reducer function. This
/// particular one increment/decrement a counter
#[derive(Clone)]
enum Msg {
    Tick,
    Key,
}

/// Reducer is called when messages are received. The idea is that you inspect
/// the message and update the model.
fn reducer(model: Model, msg: Msg) -> (Model, BatchCmd<Msg>) {
    let mut model = Model(model.0);
    match msg {
        Msg::Key => Heartbeat::stop(),
        Msg::Tick => {
            model.0 -= 1;
            if model.0 <= 0 {
                Heartbeat::stop()
            }
        }
    }
    (model, vec![])
}

/// View take data from the model and return a string which will be rendered
/// to the terminal.
fn view(model: &Model) -> String {
    format!(
        "Hi. This program will exit in {} seconds. To quit sooner press any key.",
        model.0
    )
}

/// Input is called when stdin input are received. The idea is that you inspect
/// the event and returns an optional message.
fn input(event: InputEvent) -> Option<Msg> {
    match event {
        InputEvent::Key(key) => match key {
            _ => Some(Msg::Key),
        },
        _ => None,
    }
}

// This is a subscription which we setup on runtime. It waits for one
// second, sends a tick, and then restart.
fn tick(_: &Model) -> Msg {
    thread::sleep(Duration::from_secs(1));
    Msg::Tick
}

fn main() -> Result<()> {
    let initialize = || (Model(5), None);
    moonlight::Runtime::new(reducer, initialize, input, view)
        .with_subscription(tick)
        .run()
}
