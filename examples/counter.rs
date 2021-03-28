use moonlight::quit;
use termion::event::Key;

struct Model(i32);

enum Msg {
    Increment,
    Decrement,
    Quit,
}

fn update(msg: Msg, model: &mut Model) {
    match msg {
        Msg::Quit => quit(),
        Msg::Increment => model.0 += 1,
        Msg::Decrement => model.0 -= 1,
    }
}

fn view(model: &Model) -> String {
    String::from(model.0.to_string())
}

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
