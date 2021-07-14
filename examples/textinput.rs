use std::io::Result;

use moonlight::{components::textinput, heartbeat::Heartbeat, input::InputEvent, BatchCmd, Key};

#[derive(Clone)]
struct Model {
    input: textinput::Model,
}

impl Model {
    fn new() -> Self {
        let mut model = Self {
            input: textinput::Model::new(),
        };

        model.input.placeholder = String::from("Pikachu");
        model.input.char_limit = 156;

        model
    }
}

#[derive(Clone)]
enum Msg {
    InputMsg(textinput::Message),
    Quit,
}

fn reducer(model: Model, msg: Msg) -> (Model, BatchCmd<Msg>) {
    let mut model = Model { ..model };
    match msg {
        Msg::Quit => Heartbeat::stop(),
        Msg::InputMsg(input_msg) => model.input.reducer(input_msg),
    }
    (model, vec![])
}

fn view(model: &Model) -> String {
    let help = String::from("(esc to exit)");
    format!(
        "What's is your favorite Pokemon ? \n\n{}\n\n{}",
        model.input.view(),
        help
    )
}

fn input(event: InputEvent) -> Option<Msg> {
    match event {
        InputEvent::Key(key) => match key {
            Key::Esc => Some(Msg::Quit),
            _ => Some(Msg::InputMsg(textinput::Message::Key(key))),
        },
        _ => None,
    }
}

fn blink(model: &Model) -> Msg {
    Msg::InputMsg(model.input.blink())
}

fn main() -> Result<()> {
    let initialize = || (Model::new(), None);
    moonlight::Runtime::new(reducer, initialize, input, view)
        .with_subscription(blink)
        .run()
}
