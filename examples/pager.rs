use std::fs;
use std::io::Result;

use moonlight::heartbeat::Heartbeat;
use moonlight::{
    components::viewport::{self, Message},
    input::{InputEvent, Key},
    BatchCmd, Cmd,
};
use unicode_width::UnicodeWidthStr;

const HEADER_HEIGHT: isize = 3;
const FOOTER_HEIGHT: isize = 3;

#[derive(Clone, Default)]
struct Model {
    content: String,
    ready: bool,
    viewport: viewport::Model,
}

impl Model {}

#[derive(Clone, Debug)]
enum Msg {
    Viewport(viewport::Message),
    WindowResized(u16, u16),
    Quit,
}

impl From<viewport::Message> for Msg {
    fn from(m: viewport::Message) -> Self {
        Self::Viewport(m)
    }
}

fn reducer(model: Model, msg: Msg) -> (Model, BatchCmd<Msg>) {
    let mut model = Model { ..model };
    match msg {
        Msg::Quit => Heartbeat::stop(),
        Msg::WindowResized(w, h) => {
            let (w, h) = (w as isize, h as isize);
            let vertical_margins = HEADER_HEIGHT + FOOTER_HEIGHT;

            if !model.ready {
                model.viewport = viewport::Model {
                    width: w,
                    height: h - vertical_margins,
                    ..Default::default()
                };

                model.viewport.y_position = HEADER_HEIGHT;
                model.viewport.set_content(model.content.clone());
                model.ready = true;
            } else {
                model.viewport.width = w;
                model.viewport.height = h - vertical_margins;
            }
        }
        Msg::Viewport(msg) => {
            viewport::update(msg, &mut model.viewport);
        }
    }

    (model, vec![])
}

fn view(model: &Model) -> String {
    if !model.ready {
        return "\n Initializing...".to_string();
    }

    let header_top = "╭───────────╮";
    let mut header_mid = "│ Mr. Pager ├".to_string();
    let header_bot = "╰───────────╯";
    header_mid += &"─".repeat(
        model.viewport.width as usize - UnicodeWidthStr::width(header_mid.as_str()) as usize,
    );
    let header = format!("{}\n{}\n{}", header_top, header_mid, header_bot);

    let mut footer_top = "╭──────╮".to_string();
    let mut footer_mid = format!("┤ {:.0}% │", model.viewport.scroll_percent() * 100.0);
    let mut footer_bot = "╰──────╯".to_string();

    let gap_size = model.viewport.width as usize - UnicodeWidthStr::width(footer_mid.as_str());

    footer_top = " ".repeat(gap_size) + footer_top.as_str();
    footer_mid = "─".repeat(gap_size) + footer_mid.as_str();
    footer_bot = " ".repeat(gap_size) + footer_bot.as_str();

    let footer = format!("{}\n{}\n{}", footer_top, footer_mid, footer_bot);
    format!(
        "{}\n{}\n{}",
        header,
        viewport::view(&model.viewport),
        footer
    )
}

fn input(event: InputEvent) -> Option<Msg> {
    match event {
        InputEvent::Key(key) => match key {
            Key::Char('q') | Key::Ctrl('c') | Key::Esc => Some(Msg::Quit),
            key => Some(Msg::Viewport(Message::Input(key))),
        },
        InputEvent::WindowSize { width, height } => Some(Msg::WindowResized(width, height)),
        _ => None,
    }
}

fn initialize() -> (Model, Option<Cmd<Msg>>) {
    let content =
        fs::read_to_string("examples/artichoke.md").expect("could not load file artichoke.md");
    let model = Model {
        content,
        ready: false,
        viewport: viewport::Model {
            ..Default::default()
        },
    };
    (model, None)
}

fn main() -> Result<()> {
    moonlight::Runtime::new(reducer, initialize, input, view).run()
}
