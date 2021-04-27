use std::{thread, time::Duration};

use moonlight::{quit, BatchCmd, Sub};
use termion::event::Key;

/// A simple program that counts down from 5 and then exits.

/// A model can be more or less any type of data. It holds all the data for a
/// program, so often it's a struct.
#[derive(Clone)]
struct Model(i32);

/// Messages are events that we respond to in our Update function. This
/// particular one increment/decrement a counter
enum Msg {
    Tick,
    Key,
}

/// Update is called when messages are received. The idea is that you inspect
/// the message and update the model.
fn update(msg: Msg, model: &mut Model) -> BatchCmd<Msg> {
    match msg {
        Msg::Key => quit(),
        Msg::Tick => {
            model.0 -= 1;
            if model.0 <= 0 {
                quit()
            }
        }
    }
    vec![]
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
fn input(event: Key) -> Option<Msg> {
    match event {
        _ => Some(Msg::Key),
    }
}

// This is a subscription which we setup in program(). It waits for one
// second, sends a tick, and then restart.
fn tick(_: &Model) -> Msg {
    thread::sleep(Duration::from_secs(1));
    Msg::Tick
}

fn main() {
    let subs: Vec<Sub<Model, Msg>> = vec![Box::new(tick)];
    moonlight::program(|| (Model(5), None), update, view, input, subs);
}
