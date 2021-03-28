use moonlight::quit;
use termion::event::Key;

/// A simple program that show a counter

/// A model can be more or less any type of data. It holds all the data for a
/// program, so often it's a struct.
struct Model(i32);

/// Messages are events that we respond to in our Update function. This
/// particular one increment/decrement a counter
enum Msg {
    Increment,
    Decrement,
    Quit,
}

/// Update is called when messages are received. The idea is that you inspect
/// the message and update the model.
fn update(msg: Msg, model: &mut Model) {
    match msg {
        Msg::Quit => quit(),
        Msg::Increment => model.0 += 1,
        Msg::Decrement => model.0 -= 1,
    }
}

/// View take data from the model and return a string which will be rendered
/// to the terminal.
fn view(model: &Model) -> String {
    String::from(model.0.to_string())
}

/// Input is called when stdin input are received. The idea is that you inspect
/// the event and returns an optional message.
fn input(event: Key) -> Option<Msg> {
    match event {
        Key::Char('q') => Some(Msg::Quit),
        Key::Char('k') => Some(Msg::Increment),
        Key::Char('j') => Some(Msg::Decrement),
        _ => None,
    }
}

fn main() {
    moonlight::program(Model(0), update, view, input);
}
